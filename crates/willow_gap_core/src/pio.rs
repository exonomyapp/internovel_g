//! General-purpose Private Interest Overlap Detection (`specs/pio`) and ResourceHandle registry.

use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::GapError;

pub type NamespaceId = [u8; 32];
pub type SubspaceId = [u8; 32];
pub type InterestDigest = [u8; 32];
pub type SessionCapabilityHandle = u64;

/// A canonical Willow area of interest used before salted PIO hashing.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PrivateInterest {
    pub namespace_id: NamespaceId,
    /// `None` denotes Subspace::Any (all subspaces within the namespace).
    pub subspace_id: Option<SubspaceId>,
    /// Path components (canonical UTF-8 or raw segments).
    pub path_prefix: Vec<Vec<u8>>,
}

impl PrivateInterest {
    /// Expands an interest into itself plus all relaxed parent variants (prefix ancestors
    /// and `Subspace::Any` generalizations) so peers holding overlapping areas of differing
    /// specificity can discover overlap without exposing unmatched paths.
    pub fn expand_relaxed(&self) -> Vec<PrivateInterest> {
        let mut out = BTreeSet::new();
        for len in 0..=self.path_prefix.len() {
            let prefix = self.path_prefix[..len].to_vec();
            out.insert(PrivateInterest {
                namespace_id: self.namespace_id,
                subspace_id: self.subspace_id,
                path_prefix: prefix.clone(),
            });
            out.insert(PrivateInterest {
                namespace_id: self.namespace_id,
                subspace_id: None,
                path_prefix: prefix,
            });
        }
        out.into_iter().collect()
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(64 + self.path_prefix.len() * 16);
        buf.extend_from_slice(&self.namespace_id);
        match self.subspace_id {
            Some(s) => {
                buf.push(1);
                buf.extend_from_slice(&s);
            }
            None => buf.push(0),
        }
        buf.extend_from_slice(&(self.path_prefix.len() as u32).to_be_bytes());
        for comp in &self.path_prefix {
            buf.extend_from_slice(&(comp.len() as u32).to_be_bytes());
            buf.extend_from_slice(comp);
        }
        buf
    }

    /// Computes a domain-separated salted hash (`BLAKE3` keyed by the session directional salt).
    pub fn salted_digest(&self, salt: &[u8; 32]) -> InterestDigest {
        let mut hasher = blake3::Hasher::new_keyed(salt);
        hasher.update(b"willow-pio-interest-v1");
        hasher.update(&self.canonical_bytes());
        *hasher.finalize().as_bytes()
    }

    /// Returns true if `self` is completely contained within `granted` (admission boundary check).
    pub fn is_contained_in(&self, granted: &PrivateInterest) -> bool {
        if self.namespace_id != granted.namespace_id {
            return false;
        }
        if let Some(req_sub) = granted.subspace_id {
            if self.subspace_id != Some(req_sub) {
                return false;
            }
        }
        self.path_prefix.starts_with(&granted.path_prefix)
    }
}

/// Optional enumeration capability for querying confidential subspaces inside a shared namespace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumerationGrant {
    pub namespace_id: NamespaceId,
    pub receiver_pk: SubspaceId,
    pub signature: Vec<u8>,
}

/// Tracks refcounted PIO capability handles unlocked only on confirmed mutual overlap.
#[derive(Debug)]
pub struct PioGate {
    own_salt: [u8; 32],
    peer_salt: [u8; 32],
    local_interests: Vec<PrivateInterest>,
    next_handle: SessionCapabilityHandle,
    max_handles: usize,
    granted: HashMap<SessionCapabilityHandle, PrivateInterest>,
}

impl PioGate {
    pub fn new(
        own_salt: [u8; 32],
        peer_salt: [u8; 32],
        local_interests: Vec<PrivateInterest>,
        max_handles: usize,
    ) -> Self {
        Self {
            own_salt,
            peer_salt,
            local_interests,
            next_handle: 1,
            max_handles: max_handles.max(16),
            granted: HashMap::new(),
        }
    }

    /// Produces salted digests to send to the peer; unmatched namespaces remain opaque.
    pub fn advertised_digests(&self) -> Vec<InterestDigest> {
        let mut digests = BTreeSet::new();
        for interest in &self.local_interests {
            for variant in interest.expand_relaxed() {
                digests.insert(variant.salted_digest(&self.own_salt));
            }
        }
        digests.into_iter().collect()
    }

    /// Intersects peer-advertised digests against our local expanded interests hashed with `peer_salt`.
    pub fn confirm_overlaps(&self, peer_digests: &[InterestDigest]) -> Vec<PrivateInterest> {
        let peer_set: BTreeSet<InterestDigest> = peer_digests.iter().copied().collect();
        let mut matched = Vec::new();
        for local in &self.local_interests {
            for variant in local.expand_relaxed() {
                if peer_set.contains(&variant.salted_digest(&self.peer_salt)) {
                    matched.push(variant);
                }
            }
        }
        debug!(target: "willow::pio", overlap_count = matched.len(), "confirmed PIO overlaps");
        matched
    }

    /// Registers a verified capability handle after overlap confirmation.
    pub fn register_capability(
        &mut self,
        granted_area: PrivateInterest,
    ) -> Result<SessionCapabilityHandle, GapError> {
        if self.granted.len() >= self.max_handles {
            return Err(GapError::BudgetExceeded("max_capability_handles"));
        }
        let handle = self.next_handle;
        self.next_handle += 1;
        self.granted.insert(handle, granted_area);
        Ok(handle)
    }

    /// Strict admission boundary: ensures every WTP request's grouping lies inside the handle's area.
    pub fn authorize_request(
        &self,
        handle: SessionCapabilityHandle,
        requested: &PrivateInterest,
    ) -> Result<(), GapError> {
        let area = self
            .granted
            .get(&handle)
            .ok_or(GapError::UnauthorizedHandle(handle))?;
        if !requested.is_contained_in(area) {
            warn!(target: "willow::pio", handle, "capability scope violation rejected");
            return Err(GapError::CapabilityScopeViolation);
        }
        Ok(())
    }

    pub fn release_handle(&mut self, handle: SessionCapabilityHandle) -> bool {
        self.granted.remove(&handle).is_some()
    }
}
