//! Riot-inspired Copy-on-Write Atomic Admission (`inspect -> plan -> commit`) with strict budgets.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::wtp::WtpEntryWire;
use crate::GapError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdmissionBudgets {
    pub max_entry_bytes: usize,
    pub max_capability_bytes: usize,
    pub max_path_components: usize,
    pub max_entries_per_batch: usize,
}

impl Default for AdmissionBudgets {
    fn default() -> Self {
        Self {
            max_entry_bytes: 2 * 1024 * 1024,
            max_capability_bytes: 8 * 1024,
            max_path_components: 32,
            max_entries_per_batch: 256,
        }
    }
}

/// Validated immutable batch ready for atomic installation into the local store.
#[derive(Clone, Debug)]
pub struct StagedAdmissionPlan {
    pub entries: Vec<WtpEntryWire>,
    pub total_payload_bytes: u64,
}

pub fn inspect_and_plan_batch(
    budgets: &AdmissionBudgets,
    inbound: Vec<WtpEntryWire>,
) -> Result<StagedAdmissionPlan, GapError> {
    if inbound.len() > budgets.max_entries_per_batch {
        return Err(GapError::BudgetExceeded("max_entries_per_batch"));
    }

    let mut seen_digests = HashSet::with_capacity(inbound.len());
    let mut total_payload_bytes: u64 = 0;

    for entry in &inbound {
        if entry.path.len() > budgets.max_path_components {
            return Err(GapError::BudgetExceeded("max_path_components"));
        }
        if entry.authorisation_token.len() > budgets.max_capability_bytes {
            return Err(GapError::BudgetExceeded("max_capability_bytes"));
        }
        if let Some(inline) = &entry.inline_payload {
            if inline.len() > budgets.max_entry_bytes {
                return Err(GapError::BudgetExceeded("max_entry_bytes"));
            }
            let actual_digest = *blake3::hash(inline).as_bytes();
            if actual_digest != entry.payload_digest {
                return Err(GapError::CryptoFailure);
            }
        }
        let key = (entry.namespace_id, entry.subspace_id, entry.payload_digest);
        if !seen_digests.insert(key) {
            return Err(GapError::DuplicateId);
        }
        total_payload_bytes = total_payload_bytes.saturating_add(entry.payload_length);
    }

    info!(
        target: "willow::admission",
        staged_count = inbound.len(),
        total_payload_bytes,
        "staged admission plan verified"
    );

    Ok(StagedAdmissionPlan {
        entries: inbound,
        total_payload_bytes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_corrupted_inline_digest() {
        let bad = WtpEntryWire {
            namespace_id: [1; 32],
            subspace_id: [2; 32],
            path: vec![b"chapters".to_vec(), b"01".to_vec()],
            timestamp: 100,
            payload_length: 5,
            payload_digest: [9; 32],
            inline_payload: Some(b"hello".to_vec()),
            authorisation_token: vec![1, 2, 3],
        };
        assert!(matches!(
            inspect_and_plan_batch(&AdmissionBudgets::default(), vec![bad]),
            Err(GapError::CryptoFailure)
        ));
    }
}
