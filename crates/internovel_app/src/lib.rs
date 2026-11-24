//! Shared reactive state and domain models for InterNovel's Leptos UI.

pub mod app;
pub mod markdown;
pub mod pages;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpineSortMode {
    Alphabetical,
    StorylineDate,
    LastEdited,
    Busyness,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StudioTab {
    Editor,
    PlotPlanner,
    ContactsSync,
    Gallery,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorView {
    Edit,
    Split,
    Preview,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorAccount {
    pub display_name: String,
    pub bio: String,
    pub avatar_initials: String,
    pub avatar_url: String,
    pub did_short: String,
    pub did_long: String,
    pub security_text: String,
    pub in_memory: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VertebraItem {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub title: String,
    pub storyline_date: String,
    pub last_edited_ts: u64,
    pub child_count: usize,
    pub tags: Vec<String>,
    pub content: String,
    pub plot_blocks: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryAsset {
    pub id: String,
    pub title: String,
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub size_kb: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollabPitch {
    pub id: String,
    pub author: String,
    pub text: String,
    pub date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NovelWorkspace {
    pub id: String,
    pub title: String,
    pub synopsis: String,
    pub collab_call: String,
    pub seeking_collaborators: bool,
    pub namespace_hex: String,
    pub forking_allowed: bool,
    pub forking_locked: bool,
    pub favorited: bool,
    pub remixed_from_title: Option<String>,
    pub remixed_baseline_text: Option<String>,
    pub last_vertebra_id: String,
    pub last_studio_tab: StudioTab,
    pub last_editor_view: EditorView,
    pub last_route: String,
    pub pitches: Vec<CollabPitch>,
    pub vertebrae: Vec<VertebraItem>,
    pub gallery: Vec<GalleryAsset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterTrunk {
    pub id: String,
    pub name: String,
    pub owner_name: String,
    pub owner_did: String,
    pub bio: String,
    pub sample: String,
    pub proposed_to: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactSubgroup {
    pub name: String,
    pub member_dids: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct AppState {
    pub active_account: RwSignal<Option<AuthorAccount>>,
    pub accounts: RwSignal<Vec<AuthorAccount>>,
    pub active_novel_idx: RwSignal<usize>,
    pub selected_vertebra_id: RwSignal<String>,
    pub studio_tab: RwSignal<StudioTab>,
    pub editor_view: RwSignal<EditorView>,
    pub novels: RwSignal<Vec<NovelWorkspace>>,
    pub trunks: RwSignal<Vec<CharacterTrunk>>,
    pub subgroups: RwSignal<Vec<ContactSubgroup>>,
    pub theme: RwSignal<ThemeMode>,
    pub text_scale: RwSignal<u16>,
    pub spine_collapsed: RwSignal<bool>,
    pub spine_view: RwSignal<bool>,
    pub spine_sort: RwSignal<SpineSortMode>,
    pub split_ratio: RwSignal<f64>,
    pub iroh_relay_url: RwSignal<String>,
    pub backup_schedule: RwSignal<String>,
}

impl AppState {
    pub fn snapshot_current_novel(&self, route_override: Option<&str>) {
        let cur_idx = self.active_novel_idx.get_untracked();
        let cur_v = self.selected_vertebra_id.get_untracked();
        let cur_tab = self.studio_tab.get_untracked();
        let cur_view = self.editor_view.get_untracked();
        self.novels.update(|list| {
            if let Some(nv) = list.get_mut(cur_idx) {
                if nv.vertebrae.iter().any(|v| v.id == cur_v) {
                    nv.last_vertebra_id = cur_v;
                }
                nv.last_studio_tab = cur_tab;
                nv.last_editor_view = cur_view;
                if let Some(r) = route_override {
                    nv.last_route = r.to_string();
                }
            }
        });
    }

    pub fn switch_active_novel(&self, idx: usize, route_override: Option<&str>) -> String {
        self.snapshot_current_novel(route_override);
        self.active_novel_idx.set(idx);
        let target = self.novels.get_untracked().get(idx).cloned();
        if let Some(nv) = target {
            let valid_id = if nv.vertebrae.iter().any(|v| v.id == nv.last_vertebra_id) {
                nv.last_vertebra_id.clone()
            } else {
                nv.vertebrae.first().map(|v| v.id.clone()).unwrap_or_default()
            };
            self.selected_vertebra_id.set(valid_id);
            self.studio_tab.set(nv.last_studio_tab);
            self.editor_view.set(nv.last_editor_view);
            return nv.last_route;
        }
        "/".into()
    }

    pub fn provide() -> Self {
        let default_novels = vec![
            NovelWorkspace {
                id: "novel-1".into(),
                title: "The Clockwork Archivist".into(),
                synopsis: "An apprentice archivist uncovers missing chapters across a subterranean library.".into(),
                collab_call: "Seeking a beta reader or historical architecture adviser for Act II subplots.".into(),
                seeking_collaborators: true,
                namespace_hex: "7a4f..e91c (Willow'25)".into(),
                forking_allowed: true,
                forking_locked: false,
                favorited: true,
                remixed_from_title: None,
                remixed_baseline_text: None,
                last_vertebra_id: "v-1".into(),
                last_studio_tab: StudioTab::Editor,
                last_editor_view: EditorView::Split,
                last_route: "/".into(),
                pitches: vec![CollabPitch {
                    id: "pitch-1".into(),
                    author: "Elena V.".into(),
                    text: "Happy to contribute botanical marginalia sketches for Chapter 2!".into(),
                    date: "2026-02-10".into(),
                }],
                gallery: Vec::new(),
                vertebrae: vec![
                    VertebraItem {
                        id: "v-1".into(),
                        label: "Idea · Act I".into(),
                        kind: "Idea".into(),
                        title: "Prologue: Ink on Brass".into(),
                        storyline_date: "1894-03-12".into(),
                        last_edited_ts: 170,
                        child_count: 2,
                        tags: vec!["Steampunk".into(), "Archive".into(), "Location:Clocktower".into()],
                        content: "# Prologue: Ink on Brass\n\nBegin drafting your opening scene here. **Structural Vertebrae** keep your outline organized privately while published chapters share seamlessly with collaborators.\n\n- Capture non-linear story beats in any order\n- Upload illustrations in the Gallery or `+ Image` picker to embed them in chapters".into(),
                        plot_blocks: vec![
                            "Archivist discovers a sealed brass ledger behind the east stacks".into(),
                            "Apprentice decodes the marginalia cipher before dawn".into(),
                        ],
                    },
                    VertebraItem {
                        id: "v-2".into(),
                        label: "Plot Point · Turning".into(),
                        kind: "Plot Point".into(),
                        title: "Chapter 1: The Sealed Ledger".into(),
                        storyline_date: "1894-03-18".into(),
                        last_edited_ts: 210,
                        child_count: 2,
                        tags: vec!["Mystery".into(), "Co-Authored".into(), "Character:Lyra".into()],
                        content: "# Chapter 1: The Sealed Ledger\n\nEvery contribution stays linked to the author who wrote it.\n\n> \"Ink remembers who held the nib.\"".into(),
                        plot_blocks: vec![
                            "Midnight rendezvous at the foundry".into(),
                            "Exchange Character Travel Trunk notes with co-author".into(),
                        ],
                    },
                ],
            },
            NovelWorkspace {
                id: "novel-2".into(),
                title: "Letters from Orbit".into(),
                synopsis: "Epistolary sci-fi told across deep-space relay stations.".into(),
                collab_call: "Open for co-authors writing parallel transmissions from outer belt relays.".into(),
                seeking_collaborators: true,
                namespace_hex: "b2c8..440a (Willow'25)".into(),
                forking_allowed: true,
                forking_locked: true,
                favorited: false,
                remixed_from_title: None,
                remixed_baseline_text: None,
                last_vertebra_id: "v-3".into(),
                last_studio_tab: StudioTab::Editor,
                last_editor_view: EditorView::Split,
                last_route: "/".into(),
                pitches: Vec::new(),
                gallery: Vec::new(),
                vertebrae: vec![VertebraItem {
                    id: "v-3".into(),
                    label: "Theme · Epistolary".into(),
                    kind: "Theme".into(),
                    title: "Transmission 01".into(),
                    storyline_date: "2140-09-01".into(),
                    last_edited_ts: 195,
                    child_count: 1,
                    tags: vec!["Sci-Fi".into(), "Epistolary".into(), "Location:Lagrange L4".into()],
                    content: "# Transmission 01\n\nSignal delay: 14 minutes across Lagrange point L4.".into(),
                    plot_blocks: vec!["Opening telemetry log from Relay Four".into()],
                }],
            },
        ];

        let default_trunks = vec![CharacterTrunk {
            id: "tr-1".into(),
            name: "Lyra Vance".into(),
            owner_name: "Alex Rivera".into(),
            owner_did: "did:peer:4zQmClockworkLyra".into(),
            bio: "Former clocktower cartographer carrying a brass astrolabe keyed to lost libraries.".into(),
            sample: "Lyra tapped the bezel twice until the hidden ink surfaced along the margin.".into(),
            proposed_to: None,
        }];

        let default_subgroups = vec![
            ContactSubgroup {
                name: "Editors".into(),
                member_dids: vec!["did:peer:4zQmCoAuthor1".into()],
            },
            ContactSubgroup {
                name: "Beta Readers".into(),
                member_dids: vec![],
            },
        ];

        let state = Self {
            active_account: RwSignal::new(None),
            accounts: RwSignal::new(Vec::new()),
            active_novel_idx: RwSignal::new(0),
            selected_vertebra_id: RwSignal::new("v-1".into()),
            studio_tab: RwSignal::new(StudioTab::Editor),
            editor_view: RwSignal::new(EditorView::Split),
            novels: RwSignal::new(default_novels),
            trunks: RwSignal::new(default_trunks),
            subgroups: RwSignal::new(default_subgroups),
            theme: RwSignal::new(ThemeMode::System),
            text_scale: RwSignal::new(100),
            spine_collapsed: RwSignal::new(false),
            spine_view: RwSignal::new(true),
            spine_sort: RwSignal::new(SpineSortMode::StorylineDate),
            split_ratio: RwSignal::new(50.0),
            iroh_relay_url: RwSignal::new(String::new()),
            backup_schedule: RwSignal::new("Weekly".into()),
        };
        provide_context(state);
        state
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
