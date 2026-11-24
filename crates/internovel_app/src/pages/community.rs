use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::pages::docs::{DOC_FILES, USER_FILES};
use crate::{AppState, CharacterTrunk, CollabPitch, EditorView, NovelWorkspace, StudioTab, VertebraItem};

#[derive(Clone)]
struct NetworkCatalogItem {
    id: &'static str,
    is_novel: bool,
    hop: u8,
    peer_label: &'static str,
    peer_did: &'static str,
    title: &'static str,
    summary: &'static str,
    sample: &'static str,
    tags: &'static [&'static str],
    storyline_date: &'static str,
}

const PEER_NETWORK_CATALOG: &[NetworkCatalogItem] = &[
    NetworkCatalogItem {
        id: "net-novel-astrolabe",
        is_novel: true,
        hop: 1,
        peer_label: "Elena V. (Editors subgroup)",
        peer_did: "did:peer:4zQmCoAuthorElena",
        title: "The Brass Astrolabe",
        summary: "Clockmaker siblings map forgotten transit tunnels beneath an observatory island.",
        sample: "Every gear inside the east dome carried two signatures: the guild master and the runaway.",
        tags: &["Steampunk", "Mystery", "Location:Observatory"],
        storyline_date: "1896-11-04",
    },
    NetworkCatalogItem {
        id: "net-trunk-vesper",
        is_novel: false,
        hop: 1,
        peer_label: "Elena V. (Editors subgroup)",
        peer_did: "did:peer:4zQmCoAuthorElena",
        title: "Captain Vesper Hale",
        summary: "Archivist-navigator who reads pressure gauges by ear across fogbound canals.",
        sample: "\"Turn the valve a quarter past cedar,\" Vesper murmured as the lantern dimmed.",
        tags: &["Character:Vesper", "Maritime", "Travel Trunk"],
        storyline_date: "1896-08-19",
    },
    NetworkCatalogItem {
        id: "net-novel-tidepool",
        is_novel: true,
        hop: 2,
        peer_label: "Marcus T. (Beta Readers)",
        peer_did: "did:peer:4zQmReaderMarcus",
        title: "Tidepool Cartography",
        summary: "Interlocking novellas written from three coastal weather stations during an eclipse summer.",
        sample: "When the tide retreated past Marker Nine, the sunken telegraph cable still ticked in Morse.",
        tags: &["Epistolary", "Coastal", "Location:Marker Nine"],
        storyline_date: "2028-06-14",
    },
    NetworkCatalogItem {
        id: "net-novel-underground",
        is_novel: true,
        hop: 3,
        peer_label: "Writers Guild Relay",
        peer_did: "did:peer:4zQmGuildRelay03",
        title: "Letters from the Underground Press",
        summary: "Anthology of clandestine printmakers trading ciphered broadsheets across orbital colonies.",
        sample: "No censor could confiscate lead type once the apprentices memorized every galley proof.",
        tags: &["Sci-Fi", "Location:Ceres Station"],
        storyline_date: "2144-02-22",
    },
];

#[component]
pub fn CastingPage() -> impl IntoView {
    let state = expect_context::<AppState>();
    let char_name = RwSignal::new(String::new());
    let char_bio = RwSignal::new(String::new());
    let char_sample = RwSignal::new(String::new());
    let editing_trunk_id = RwSignal::new(Option::<String>::None);
    let edit_name = RwSignal::new(String::new());
    let edit_bio = RwSignal::new(String::new());
    let edit_sample = RwSignal::new(String::new());
    let confirm_del_id = RwSignal::new(Option::<String>::None);
    let proposal_note = RwSignal::new(String::new());

    let publish_trunk = move |_| {
        let name = char_name.get().trim().to_string();
        if name.is_empty() {
            return;
        }
        let owner_name = state
            .active_account
            .get()
            .map(|a| a.display_name)
            .unwrap_or_else(|| "Local Author".into());
        let owner_did = state
            .active_account
            .get()
            .map(|a| a.did_short)
            .unwrap_or_else(|| "did:peer:4zQmLocalAuthor".into());
        let sample_raw = char_sample.get().trim().to_string();
        let sample = if sample_raw.is_empty() {
            "Character travel trunk narrative sample.".into()
        } else {
            sample_raw
        };
        state.trunks.update(|list| {
            list.push(CharacterTrunk {
                id: format!("tr-{}", list.len() + 1),
                name,
                owner_name,
                owner_did,
                bio: char_bio.get(),
                sample,
                proposed_to: None,
            });
        });
        char_name.set(String::new());
        char_bio.set(String::new());
        char_sample.set(String::new());
    };

    view! {
        <div class="space-y-6">
            <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-3">
                <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">"Casting Studio & Character Travel Trunks"</h1>
                <p class="text-xs opacity-80">
                    "Showcase portable characters with narrative samples, edit their biographies, and propose cross-story appearances."
                </p>
                <div class="grid gap-2 md:grid-cols-4 text-xs pt-2">
                    <input
                        type="text"
                        placeholder="Character Name"
                        class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                        prop:value=move || char_name.get()
                        on:input=move |ev| char_name.set(event_target_value(&ev))
                    />
                    <input
                        type="text"
                        placeholder="Short Bio"
                        class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                        prop:value=move || char_bio.get()
                        on:input=move |ev| char_bio.set(event_target_value(&ev))
                    />
                    <input
                        type="text"
                        placeholder="Narrative Sample"
                        class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                        prop:value=move || char_sample.get()
                        on:input=move |ev| char_sample.set(event_target_value(&ev))
                    />
                    <button type="button" class="rounded-lg bg-[#5A5A40] text-white px-3 py-1.5 font-medium" on:click=publish_trunk>
                        "Publish Travel Trunk"
                    </button>
                </div>
            </div>

            <div class="grid gap-4 md:grid-cols-2">
                <For
                    each=move || state.trunks.get()
                    key=|t| t.id.clone()
                    children=move |trunk| {
                        let tid = trunk.id.clone();
                        let tid_edit = tid.clone();
                        let tid_save = tid.clone();
                        let tid_del = tid.clone();
                        let tid_prop = tid.clone();
                        let tid_withdraw = tid.clone();
                        let name_init = trunk.name.clone();
                        let bio_init = trunk.bio.clone();
                        let sample_init = trunk.sample.clone();
                        let target_novel = RwSignal::new(
                            state.novels.get_untracked().get(state.active_novel_idx.get_untracked()).map(|n| n.title).unwrap_or_default()
                        );

                        view! {
                            <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-2.5 text-xs">
                                {move || if editing_trunk_id.get().as_deref() == Some(tid_edit.as_str()) {
                                    let t_s = tid_save.clone();
                                    view! {
                                        <div class="space-y-2">
                                            <input
                                                type="text"
                                                class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1 font-medium"
                                                prop:value=move || edit_name.get()
                                                on:input=move |ev| edit_name.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1"
                                                prop:value=move || edit_bio.get()
                                                on:input=move |ev| edit_bio.set(event_target_value(&ev))
                                            />
                                            <textarea
                                                rows="2"
                                                class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1"
                                                prop:value=move || edit_sample.get()
                                                on:input=move |ev| edit_sample.set(event_target_value(&ev))
                                            />
                                            <div class="flex gap-2">
                                                <button
                                                    type="button"
                                                    class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white"
                                                    on:click=move |_| {
                                                        state.trunks.update(|list| {
                                                            if let Some(tr) = list.iter_mut().find(|x| x.id == t_s) {
                                                                let n = edit_name.get().trim().to_string();
                                                                if !n.is_empty() { tr.name = n; }
                                                                tr.bio = edit_bio.get().trim().to_string();
                                                                let s = edit_sample.get().trim().to_string();
                                                                if !s.is_empty() { tr.sample = s; }
                                                            }
                                                        });
                                                        editing_trunk_id.set(None);
                                                    }
                                                >"Save Changes"</button>
                                                <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| editing_trunk_id.set(None)>"Cancel"</button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    let trunk_view = trunk.clone();
                                    view! {
                                        {
                                            let tr_card = trunk_view.clone();
                                            let n_i = tr_card.name.clone();
                                            let b_i = tr_card.bio.clone();
                                            let s_i = tr_card.sample.clone();
                                            let t_id_e = tr_card.id.clone();
                                            let t_id_d = tr_card.id.clone();
                                            let t_id_p = tr_card.id.clone();
                                            let t_id_w = tr_card.id.clone();
                                            let proposed = tr_card.proposed_to.clone();
                                            view! {
                                                <div class="flex items-start justify-between gap-2">
                                                    <div>
                                                        <div class="font-serif text-base font-bold text-[#5A5A40] dark:text-[#e6e6de]">{tr_card.name.clone()}</div>
                                                        <div class="text-[11px] opacity-70">{format!("by {} · {}", tr_card.owner_name, tr_card.owner_did)}</div>
                                                    </div>
                                                    <div class="flex items-center gap-1">
                                                        <button
                                                            type="button"
                                                            class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                            on:click=move |_| {
                                                                edit_name.set(n_i.clone());
                                                                edit_bio.set(b_i.clone());
                                                                edit_sample.set(s_i.clone());
                                                                editing_trunk_id.set(Some(t_id_e.clone()));
                                                            }
                                                        >"Edit"</button>
                                                        {move || if confirm_del_id.get().as_deref() == Some(t_id_d.as_str()) {
                                                            let del_target = t_id_d.clone();
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class="px-2 py-0.5 rounded bg-red-700 text-white"
                                                                    on:click=move |_| {
                                                                        state.trunks.update(|list| list.retain(|x| x.id != del_target));
                                                                        confirm_del_id.set(None);
                                                                    }
                                                                >"Confirm"</button>
                                                                <button type="button" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| confirm_del_id.set(None)>"Cancel"</button>
                                                            }.into_any()
                                                        } else {
                                                            let trig = t_id_d.clone();
                                                            view! {
                                                                <button type="button" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400" on:click=move |_| confirm_del_id.set(Some(trig.clone()))>"Delete"</button>
                                                            }.into_any()
                                                        }}
                                                    </div>
                                                </div>
                                                <p>{tr_card.bio.clone()}</p>
                                                <blockquote class="border-l-2 border-[#5A5A40] pl-3 italic opacity-85">{tr_card.sample.clone()}</blockquote>
                                                <div class="flex flex-wrap items-center gap-1.5 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a]">
                                                    <select
                                                        class="rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1 text-[11px]"
                                                        on:change=move |ev| target_novel.set(event_target_value(&ev))
                                                    >
                                                        {move || state.novels.get().into_iter().map(|nv| {
                                                            view! { <option value=nv.title.clone()>{nv.title}</option> }
                                                        }).collect_view()}
                                                    </select>
                                                    <button
                                                        type="button"
                                                        class="px-2.5 py-1 rounded-lg bg-[#5A5A40] text-white font-medium"
                                                        on:click=move |_| {
                                                            let target = target_novel.get();
                                                            if target.is_empty() { return; }
                                                            state.trunks.update(|list| {
                                                                if let Some(tr) = list.iter_mut().find(|x| x.id == t_id_p) {
                                                                    tr.proposed_to = Some(target.clone());
                                                                    proposal_note.set(format!("Prepared character proposal for '{}' -> \"{}\".", tr.name, target));
                                                                }
                                                            });
                                                        }
                                                    >{if proposed.is_some() { "Re-Propose" } else { "Propose for Novel" }}</button>
                                                    {proposed.map(|p| view! {
                                                        <button
                                                            type="button"
                                                            class="px-2 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                            on:click=move |_| {
                                                                state.trunks.update(|list| {
                                                                    if let Some(tr) = list.iter_mut().find(|x| x.id == t_id_w) {
                                                                        tr.proposed_to = None;
                                                                    }
                                                                });
                                                            }
                                                        >{format!("Withdraw ({p})")}</button>
                                                    })}
                                                </div>
                                            }
                                        }
                                    }.into_any()
                                }}
                            </div>
                        }
                    }
                />
            </div>

            <Show when=move || !proposal_note.get().is_empty()>
                <p class="text-xs font-medium text-[#5A5A40] dark:text-[#d0d0c8]">{move || proposal_note.get()}</p>
            </Show>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CafeFilter {
    All,
    Collab,
    Forkable,
    Remixes,
}

#[component]
pub fn CafePage() -> impl IntoView {
    let state = expect_context::<AppState>();
    let navigate = use_navigate();
    let filter = RwSignal::new(CafeFilter::All);
    let remix_open_for = RwSignal::new(Option::<String>::None);
    let editing_id = RwSignal::new(Option::<String>::None);
    let edit_title = RwSignal::new(String::new());
    let edit_syn = RwSignal::new(String::new());
    let edit_call = RwSignal::new(String::new());
    let edit_seeking = RwSignal::new(false);
    let confirm_del_id = RwSignal::new(Option::<String>::None);

    let filtered_novels = move || {
        state
            .novels
            .get()
            .into_iter()
            .filter(|n| match filter.get() {
                CafeFilter::All => true,
                CafeFilter::Collab => n.seeking_collaborators,
                CafeFilter::Forkable => n.forking_allowed,
                CafeFilter::Remixes => n.remixed_from_title.is_some(),
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="space-y-6 text-xs">
            <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-3">
                <div class="flex flex-wrap items-center justify-between gap-2">
                    <div>
                        <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">"Novel Café"</h1>
                        <p class="opacity-80">
                            "Featured novels, open co-author invitations, pitches, and remix lineage attribution."
                        </p>
                    </div>
                    <div class="flex flex-wrap gap-1">
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| filter.set(CafeFilter::All)>"All"</button>
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| filter.set(CafeFilter::Collab)>"Open Co-Author Calls"</button>
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| filter.set(CafeFilter::Forkable)>"Forkable"</button>
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| filter.set(CafeFilter::Remixes)>"Remixes"</button>
                    </div>
                </div>
            </div>

            <div class="grid gap-4 md:grid-cols-2">
                <For
                    each=filtered_novels
                    key=|n| n.id.clone()
                    children=move |novel| {
                        let nid = novel.id.clone();
                        let nid_edit = nid.clone();
                        let nid_save = nid.clone();
                        let nid_del = nid.clone();
                        let nid_open = nid.clone();
                        let nid_fork = nid.clone();
                        let nid_diff = nid.clone();
                        let nid_pitch = nid.clone();
                        let pitch_draft = RwSignal::new(String::new());
                        let nav_open = navigate.clone();
                        let orig_text = novel.remixed_baseline_text.clone().unwrap_or_else(|| {
                            novel.vertebrae.first().map(|v| v.content.clone()).unwrap_or_default()
                        });
                        let cur_text = novel.vertebrae.first().map(|v| v.content.clone()).unwrap_or_default();
                        let title_init = novel.title.clone();
                        let syn_init = novel.synopsis.clone();
                        let call_init = novel.collab_call.clone();
                        let seek_init = novel.seeking_collaborators;
                        let forking_allowed = novel.forking_allowed;
                        let pitches = novel.pitches.clone();
                        let pitch_count = pitches.len();

                        view! {
                            <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-3">
                                {move || if editing_id.get().as_deref() == Some(nid_edit.as_str()) {
                                    let ns = nid_save.clone();
                                    view! {
                                        <div class="space-y-2 border-b border-[#e5e5e0] dark:border-[#2d2d2a] pb-3">
                                            <input type="text" class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5 font-serif font-bold" prop:value=move || edit_title.get() on:input=move |ev| edit_title.set(event_target_value(&ev))/>
                                            <textarea rows="2" class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5" prop:value=move || edit_syn.get() on:input=move |ev| edit_syn.set(event_target_value(&ev))/>
                                            <input type="text" placeholder="Collaborator call note..." class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5" prop:value=move || edit_call.get() on:input=move |ev| edit_call.set(event_target_value(&ev))/>
                                            <label class="inline-flex items-center gap-1.5">
                                                <input type="checkbox" prop:checked=move || edit_seeking.get() on:change=move |ev| edit_seeking.set(event_target_checked(&ev))/>
                                                <span>"Seeking Collaborators"</span>
                                            </label>
                                            <div class="flex gap-2">
                                                <button
                                                    type="button"
                                                    class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white"
                                                    on:click=move |_| {
                                                        state.novels.update(|list| {
                                                            if let Some(target) = list.iter_mut().find(|x| x.id == ns) {
                                                                let t = edit_title.get().trim().to_string();
                                                                if !t.is_empty() { target.title = t; }
                                                                target.synopsis = edit_syn.get().trim().to_string();
                                                                target.collab_call = edit_call.get().trim().to_string();
                                                                target.seeking_collaborators = edit_seeking.get();
                                                            }
                                                        });
                                                        editing_id.set(None);
                                                    }
                                                >"Save"</button>
                                                <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| editing_id.set(None)>"Cancel"</button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    let id_e = nid_edit.clone();
                                    let id_d = nid_del.clone();
                                    let t_i = title_init.clone();
                                    let s_i = syn_init.clone();
                                    let c_i = call_init.clone();
                                    let forks_allowed = novel.forking_allowed;
                                    let forks_locked = novel.forking_locked;
                                    let seeking_collab = novel.seeking_collaborators;
                                    view! {
                                        <div class="flex items-start justify-between gap-2">
                                            <div>
                                                <div class="font-serif text-base font-bold text-[#5A5A40] dark:text-[#e6e6de]">{t_i.clone()}</div>
                                                <div class="text-[11px] opacity-75">
                                                    {if forks_allowed {
                                                        if forks_locked { "Forking Enabled (Locked)" } else { "Forking Enabled" }
                                                    } else { "Forking Disabled" }}
                                                    {if seeking_collab { " · Seeking Collaborators" } else { "" }}
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-1">
                                                <button
                                                    type="button"
                                                    class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                    on:click=move |_| {
                                                        edit_title.set(t_i.clone());
                                                        edit_syn.set(s_i.clone());
                                                        edit_call.set(c_i.clone());
                                                        edit_seeking.set(seek_init);
                                                        editing_id.set(Some(id_e.clone()));
                                                    }
                                                >"Edit"</button>
                                                {move || if confirm_del_id.get().as_deref() == Some(id_d.as_str()) {
                                                    let dt = id_d.clone();
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class="px-2 py-0.5 rounded bg-red-700 text-white"
                                                            on:click=move |_| {
                                                                state.novels.update(|list| list.retain(|x| x.id != dt));
                                                                let len = state.novels.get().len();
                                                                let idx = state.active_novel_idx.get().min(len.saturating_sub(1));
                                                                state.active_novel_idx.set(idx);
                                                                confirm_del_id.set(None);
                                                            }
                                                        >"Confirm"</button>
                                                        <button type="button" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| confirm_del_id.set(None)>"Cancel"</button>
                                                    }.into_any()
                                                } else {
                                                    let trig = id_d.clone();
                                                    view! {
                                                        <button type="button" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400" on:click=move |_| confirm_del_id.set(Some(trig.clone()))>"Delete"</button>
                                                    }.into_any()
                                                }}
                                            </div>
                                        </div>
                                        <p class="opacity-85">{syn_init.clone()}</p>
                                    }.into_any()
                                }}

                                <div class="flex flex-wrap gap-2 pt-1">
                                    <button
                                        type="button"
                                        class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                        on:click=move |_| {
                                            if let Some(idx) = state.novels.get().iter().position(|x| x.id == nid_open) {
                                                state.switch_active_novel(idx, Some("/cafe"));
                                                nav_open("/", Default::default());
                                            }
                                        }
                                    >"Open in Studio"</button>
                                    <Show when=move || forking_allowed>
                                        <button
                                            type="button"
                                            class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white"
                                            on:click=move |_| {
                                                state.snapshot_current_novel(Some("/cafe"));
                                                let mut forked_opt = None;
                                                state.novels.update(|list| {
                                                    if let Some(orig) = list.iter_mut().find(|x| x.id == nid_fork) {
                                                        orig.forking_locked = true;
                                                        let baseline = orig.vertebrae.first().map(|v| v.content.clone()).unwrap_or_default();
                                                        let first_id = orig.vertebrae.first().map(|v| v.id.clone()).unwrap_or_default();
                                                        let mut forked = orig.clone();
                                                        forked.id = format!("novel-fork-{}", list.len() + 1);
                                                        forked.title = format!("{} (Remix)", orig.title);
                                                        forked.forking_locked = false;
                                                        forked.remixed_from_title = Some(orig.title.clone());
                                                        forked.remixed_baseline_text = Some(baseline);
                                                        forked.last_vertebra_id = first_id;
                                                        forked.last_studio_tab = StudioTab::Editor;
                                                        forked.last_editor_view = EditorView::Split;
                                                        forked.last_route = "/cafe".into();
                                                        forked.pitches.clear();
                                                        forked_opt = Some(forked);
                                                    }
                                                    if let Some(f) = forked_opt {
                                                        list.push(f);
                                                        state.active_novel_idx.set(list.len() - 1);
                                                    }
                                                });
                                            }
                                        >"Fork / Remix Novel"</button>
                                    </Show>
                                    <button
                                        type="button"
                                        class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                        on:click=move |_| {
                                            let cur = remix_open_for.get();
                                            remix_open_for.set(if cur.as_deref() == Some(nid_diff.as_str()) { None } else { Some(nid_diff.clone()) });
                                        }
                                    >"Inspect Remix Lineage Diff"</button>
                                </div>

                                <Show when=move || remix_open_for.get().as_deref() == Some(nid.as_str())>
                                    {
                                        let o_t = orig_text.clone();
                                        let c_t = cur_text.clone();
                                        view! {
                                            <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-3 space-y-1">
                                                <div class="font-semibold">"Remix Lineage Comparison"</div>
                                                {if o_t != c_t {
                                                    view! {
                                                        <p class="text-red-700 dark:text-red-400 line-through">{format!("- Original: {}", o_t)}</p>
                                                        <p class="text-emerald-700 dark:text-emerald-400">{format!("+ Remixed: {}", c_t)}</p>
                                                    }.into_any()
                                                } else {
                                                    view! { <p class="opacity-75">"No prose divergence yet. Fork and edit Chapter 1 in Studio to inspect live differences."</p> }.into_any()
                                                }}
                                            </div>
                                        }
                                    }
                                </Show>

                                <div class="pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] space-y-1.5">
                                    <div class="font-medium opacity-80">{format!("Collaboration Pitches ({})", pitch_count)}</div>
                                    {pitches.into_iter().enumerate().map(|(p_idx, pitch)| {
                                        let nid_p_del = nid_pitch.clone();
                                        view! {
                                            <div class="flex items-start justify-between gap-2 rounded-lg bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5">
                                                <span>{format!("{}: {} ({})", pitch.author, pitch.text, pitch.date)}</span>
                                                <button
                                                    type="button"
                                                    class="text-red-700 dark:text-red-400"
                                                    on:click=move |_| {
                                                        state.novels.update(|list| {
                                                            if let Some(target) = list.iter_mut().find(|x| x.id == nid_p_del) {
                                                                if p_idx < target.pitches.len() { target.pitches.remove(p_idx); }
                                                            }
                                                        });
                                                    }
                                                >"×"</button>
                                            </div>
                                        }
                                    }).collect_view()}
                                    <div class="flex gap-1.5">
                                        <input
                                            type="text"
                                            placeholder="Offer a co-author pitch..."
                                            class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1"
                                            prop:value=move || pitch_draft.get()
                                            on:input=move |ev| pitch_draft.set(event_target_value(&ev))
                                        />
                                        <button
                                            type="button"
                                            class="px-2.5 py-1 rounded-lg bg-[#5A5A40] text-white"
                                            on:click=move |_| {
                                                let txt = pitch_draft.get().trim().to_string();
                                                if txt.is_empty() { return; }
                                                let author = state.active_account.get().map(|a| a.display_name).unwrap_or_else(|| "Guest Author".into());
                                                let nid_add = nid_pitch.clone();
                                                state.novels.update(|list| {
                                                    if let Some(target) = list.iter_mut().find(|x| x.id == nid_add) {
                                                        target.pitches.push(CollabPitch {
                                                            id: format!("pitch-{}", target.pitches.len() + 1),
                                                            author,
                                                            text: txt,
                                                            date: "2026-03-01".into(),
                                                        });
                                                    }
                                                });
                                                pitch_draft.set(String::new());
                                            }
                                        >"Post Pitch"</button>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[derive(Clone)]
enum SearchTarget {
    Studio {
        novel_idx: usize,
        vertebra_id: String,
        tab: StudioTab,
    },
    Casting,
    Doc(String),
}

#[derive(Clone)]
struct SearchHit {
    badge: &'static str,
    title: String,
    subtitle: String,
    excerpt: String,
    score: i32,
    storyline_date: String,
    novel_id: Option<String>,
    favorited: bool,
    target: SearchTarget,
}

fn match_terms_score(terms: &[String], title: &str, body_fields: &[&str], tags: &[String]) -> i32 {
    if terms.is_empty() {
        return 10;
    }
    let title_l = title.to_lowercase();
    let tags_l = tags.join(" ").to_lowercase();
    let combined = format!("{title_l} {tags_l} {}", body_fields.join(" ").to_lowercase());
    if !terms.iter().all(|t| combined.contains(t)) {
        return 0;
    }
    let mut score = 10;
    for t in terms {
        if title_l == *t {
            score += 60;
        } else if title_l.contains(t) {
            score += 35;
        }
        if tags_l.contains(t) {
            score += 25;
        }
    }
    score
}

fn build_excerpt(raw: &str, terms: &[String]) -> String {
    let clean: String = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if clean.is_empty() {
        return String::new();
    }
    let lower = clean.to_lowercase();
    let byte_hit = terms
        .iter()
        .filter_map(|t| lower.find(t))
        .min()
        .unwrap_or(0);
    let mut start = byte_hit.saturating_sub(40);
    while start > 0 && !clean.is_char_boundary(start) {
        start -= 1;
    }
    let mut end = (start + 150).min(clean.len());
    while end < clean.len() && !clean.is_char_boundary(end) {
        end += 1;
    }
    let prefix = if start > 0 { "…" } else { "" };
    let suffix = if end < clean.len() { "…" } else { "" };
    format!("{prefix}{}{suffix}", &clean[start..end])
}

#[component]
pub fn SearchPage() -> impl IntoView {
    let state = expect_context::<AppState>();
    let navigate = use_navigate();
    let query = RwSignal::new(String::new());
    let novel_scope = RwSignal::new("all".to_string());
    let sort_mode = RwSignal::new("relevance".to_string());
    let cat_chapters = RwSignal::new(true);
    let cat_characters = RwSignal::new(true);
    let cat_plot = RwSignal::new(true);
    let cat_themes = RwSignal::new(true);
    let cat_locations = RwSignal::new(true);
    let cat_images = RwSignal::new(true);
    let cat_docs = RwSignal::new(true);
    let gossip_depth = RwSignal::new(2u8);
    let gossip_status = RwSignal::new(String::new());
    let gossip_hits = RwSignal::new(Vec::<NetworkCatalogItem>::new());
    let imported_ids = RwSignal::new(Vec::<&'static str>::new());

    let run_network_query = move || {
        let q = query.get();
        let terms: Vec<String> = q
            .trim()
            .to_lowercase()
            .split_whitespace()
            .map(String::from)
            .collect();
        let depth = gossip_depth.get();
        let matches: Vec<_> = PEER_NETWORK_CATALOG
            .iter()
            .filter(|item| {
                if item.hop > depth {
                    return false;
                }
                let tags: Vec<String> = item.tags.iter().map(|s| s.to_string()).collect();
                match_terms_score(&terms, item.title, &[item.summary, item.sample, item.peer_label], &tags) > 0
            })
            .cloned()
            .collect();
        let label = if q.trim().is_empty() { "all peer stories".to_string() } else { q.trim().to_string() };
        gossip_status.set(if depth == 0 {
            "Network Depth is 0 (local-only mode). Increase depth to 1–4 to query nearby collaborators.".into()
        } else {
            format!(
                "Queried peers up to {} hop(s) for \"{}\" — {} result(s) found.",
                depth,
                label,
                matches.len()
            )
        });
        gossip_hits.set(matches);
    };

    view! {
        <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-4 text-xs">
            <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">"Local-First & Gossip Search"</h1>
            <div class="flex flex-wrap gap-2">
                <input
                    type="text"
                    placeholder="Search novels, chapters, characters, plot cards, or docs..."
                    class="flex-1 min-w-[200px] rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                />
                <select
                    class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5"
                    on:change=move |ev| novel_scope.set(event_target_value(&ev))
                >
                    <option value="all">"All Novels & Community"</option>
                    {move || state.novels.get().into_iter().map(|n| view! { <option value=n.id.clone()>{n.title}</option> }).collect_view()}
                </select>
                <select
                    class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5"
                    on:change=move |ev| sort_mode.set(event_target_value(&ev))
                >
                    <option value="relevance">"Sort: Best Match"</option>
                    <option value="date">"Sort: Storyline Date"</option>
                    <option value="alpha">"Sort: Alphabetical"</option>
                </select>
                <label class="flex items-center gap-1.5">
                    <span>"Network Depth:"</span>
                    <input
                        type="number"
                        min="0"
                        max="4"
                        class="w-14 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1"
                        prop:value=move || gossip_depth.get().to_string()
                        on:input=move |ev| {
                            if let Ok(d) = event_target_value(&ev).parse::<u8>() {
                                gossip_depth.set(d.min(4));
                            }
                        }
                    />
                </label>
                <button
                    type="button"
                    class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium"
                    on:click=move |_| run_network_query()
                >"Send Network Query"</button>
            </div>

            <div class="flex flex-wrap gap-3">
                <label class="inline-flex items-center gap-1"><input type="checkbox" prop:checked=move || cat_chapters.get() on:change=move |ev| cat_chapters.set(event_target_checked(&ev))/><span>"Chapters"</span></label>
                <label class="inline-flex items-center gap-1"><input type="checkbox" prop:checked=move || cat_characters.get() on:change=move |ev| cat_characters.set(event_target_checked(&ev))/><span>"Characters"</span></label>
                <label class="inline-flex items-center gap-1"><input type="checkbox" prop:checked=move || cat_plot.get() on:change=move |ev| cat_plot.set(event_target_checked(&ev))/><span>"Plot Cards"</span></label>
                <label class="inline-flex items-center gap-1"><input type="checkbox" prop:checked=move || cat_themes.get() on:change=move |ev| cat_themes.set(event_target_checked(&ev))/><span>"Themes & Tags"</span></label>
                <label class="inline-flex items-center gap-1"><input type="checkbox" prop:checked=move || cat_locations.get() on:change=move |ev| cat_locations.set(event_target_checked(&ev))/><span>"Locations"</span></label>
                <label class="inline-flex items-center gap-1"><input type="checkbox" prop:checked=move || cat_images.get() on:change=move |ev| cat_images.set(event_target_checked(&ev))/><span>"Images"</span></label>
                <label class="inline-flex items-center gap-1"><input type="checkbox" prop:checked=move || cat_docs.get() on:change=move |ev| cat_docs.set(event_target_checked(&ev))/><span>"Docs & Guides"</span></label>
            </div>

            <Show when=move || !gossip_status.get().is_empty()>
                <p class="rounded-lg bg-[#f5f5f0] dark:bg-[#252522] px-3 py-2 text-[#5A5A40] dark:text-[#d0d0c8] font-medium">{move || gossip_status.get()}</p>
            </Show>

            <div class="space-y-2">
                {move || {
                    let terms: Vec<String> = query.get().trim().to_lowercase().split_whitespace().map(String::from).collect();
                    let scope = novel_scope.get();
                    let mut hits = Vec::<SearchHit>::new();

                    for (n_idx, n) in state.novels.get().into_iter().enumerate() {
                        if scope != "all" && n.id != scope { continue; }
                        for v in &n.vertebrae {
                            let has_char = v.tags.iter().any(|t| t.starts_with("Character:"));
                            let has_loc = v.tags.iter().any(|t| t.starts_with("Location:"));
                            let has_theme = v.tags.iter().any(|t| !t.starts_with("Character:") && !t.starts_with("Location:"));
                            let allow_ch = cat_chapters.get()
                                || (cat_characters.get() && has_char)
                                || (cat_locations.get() && has_loc)
                                || (cat_themes.get() && has_theme);

                            if allow_ch {
                                let sc = match_terms_score(&terms, &v.title, &[&n.title, &n.synopsis, &v.kind, &v.content], &v.tags);
                                if sc > 0 {
                                    hits.push(SearchHit {
                                        badge: "Chapter",
                                        title: format!("{} — {}", n.title, v.title),
                                        subtitle: format!("{} · Storyline: {}", v.kind, v.storyline_date),
                                        excerpt: build_excerpt(&v.content, &terms),
                                        score: sc,
                                        storyline_date: v.storyline_date.clone(),
                                        novel_id: Some(n.id.clone()),
                                        favorited: n.favorited,
                                        target: SearchTarget::Studio { novel_idx: n_idx, vertebra_id: v.id.clone(), tab: StudioTab::Editor },
                                    });
                                }
                            }
                            if cat_plot.get() {
                                for (b_i, block) in v.plot_blocks.iter().enumerate() {
                                    let sc = match_terms_score(&terms, block, &[&n.title, &v.title], &v.tags);
                                    if sc > 0 {
                                        hits.push(SearchHit {
                                            badge: "Plot Card",
                                            title: block.clone(),
                                            subtitle: format!("Plot Planner in {} · {} (Beat {})", n.title, v.title, b_i + 1),
                                            excerpt: build_excerpt(&v.content, &terms),
                                            score: sc + 5,
                                            storyline_date: v.storyline_date.clone(),
                                            novel_id: Some(n.id.clone()),
                                            favorited: n.favorited,
                                            target: SearchTarget::Studio { novel_idx: n_idx, vertebra_id: v.id.clone(), tab: StudioTab::PlotPlanner },
                                        });
                                    }
                                }
                            }
                        }

                        if cat_images.get() {
                            for img in &n.gallery {
                                let tags = vec!["Gallery Asset".to_string()];
                                let sc = match_terms_score(&terms, &img.title, &[&n.title], &tags);
                                if sc > 0 {
                                    let v_first = n.vertebrae.first().map(|x| x.id.clone()).unwrap_or_default();
                                    hits.push(SearchHit {
                                        badge: "Image",
                                        title: img.title.clone(),
                                        subtitle: format!("{} Gallery · {}×{}", n.title, img.width, img.height),
                                        excerpt: format!("![{}](internovel-img://{})", img.title, img.id),
                                        score: sc,
                                        storyline_date: "9999-12-31".into(),
                                        novel_id: Some(n.id.clone()),
                                        favorited: n.favorited,
                                        target: SearchTarget::Studio { novel_idx: n_idx, vertebra_id: v_first, tab: StudioTab::Gallery },
                                    });
                                }
                            }
                        }
                    }

                    if cat_characters.get() && scope == "all" {
                        for t in state.trunks.get() {
                            let tags = vec!["Character".into(), "Travel Trunk".into()];
                            let sc = match_terms_score(&terms, &t.name, &[&t.bio, &t.sample, &t.owner_name], &tags);
                            if sc > 0 {
                                hits.push(SearchHit {
                                    badge: "Character",
                                    title: t.name.clone(),
                                    subtitle: format!("Character Travel Trunk by {}", t.owner_name),
                                    excerpt: build_excerpt(&format!("{} — {}", t.bio, t.sample), &terms),
                                    score: sc + 10,
                                    storyline_date: "0000-00-00".into(),
                                    novel_id: None,
                                    favorited: false,
                                    target: SearchTarget::Casting,
                                });
                            }
                        }
                    }

                    if cat_docs.get() && scope == "all" {
                        for (slug, title, body) in DOC_FILES {
                            let tags = vec!["docs".to_string()];
                            let sc = match_terms_score(&terms, title, &[body], &tags);
                            if sc > 0 {
                                hits.push(SearchHit {
                                    badge: "Doc",
                                    title: (*title).to_string(),
                                    subtitle: "Documentation".into(),
                                    excerpt: build_excerpt(body, &terms),
                                    score: sc - 5,
                                    storyline_date: "0000-00-00".into(),
                                    novel_id: None,
                                    favorited: false,
                                    target: SearchTarget::Doc(format!("/docs/{slug}")),
                                });
                            }
                        }
                        for (slug, title, body) in USER_FILES {
                            let tags = vec!["user".to_string()];
                            let sc = match_terms_score(&terms, title, &[body], &tags);
                            if sc > 0 {
                                hits.push(SearchHit {
                                    badge: "Doc",
                                    title: (*title).to_string(),
                                    subtitle: "User Guide".into(),
                                    excerpt: build_excerpt(body, &terms),
                                    score: sc - 5,
                                    storyline_date: "0000-00-00".into(),
                                    novel_id: None,
                                    favorited: false,
                                    target: SearchTarget::Doc(format!("/user/{slug}")),
                                });
                            }
                        }
                    }

                    match sort_mode.get().as_str() {
                        "alpha" => hits.sort_by(|a, b| a.title.cmp(&b.title)),
                        "date" => hits.sort_by(|a, b| b.storyline_date.cmp(&a.storyline_date)),
                        _ => hits.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.title.cmp(&b.title))),
                    }

                    if hits.is_empty() {
                        return view! { <p class="opacity-70 py-3">"No matching local items found for those filters."</p> }.into_any();
                    }

                    hits.into_iter().map(|h| {
                        let nav = navigate.clone();
                        let tgt = h.target.clone();
                        let fav_nid = h.novel_id.clone();
                        view! {
                            <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-3.5 space-y-1.5">
                                <div class="flex flex-wrap items-start justify-between gap-2">
                                    <div>
                                        <span class="px-2 py-0.5 rounded bg-[#f5f5f0] dark:bg-[#252522] text-[10px] uppercase font-medium mr-2">{h.badge}</span>
                                        <span class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">{h.title}</span>
                                        <div class="text-[11px] opacity-70">{h.subtitle}</div>
                                    </div>
                                    <div class="flex items-center gap-1.5">
                                        {fav_nid.map(|nid| view! {
                                            <button
                                                type="button"
                                                class="px-2.5 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                on:click=move |_| {
                                                    state.novels.update(|list| {
                                                        if let Some(nv) = list.iter_mut().find(|x| x.id == nid) {
                                                            nv.favorited = !nv.favorited;
                                                        }
                                                    });
                                                }
                                            >{if h.favorited { "Synced" } else { "Favorite" }}</button>
                                        })}
                                        <button
                                            type="button"
                                            class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white font-medium"
                                            on:click=move |_| match &tgt {
                                                SearchTarget::Studio { novel_idx, vertebra_id, tab } => {
                                                    state.switch_active_novel(*novel_idx, Some("/search"));
                                                    if !vertebra_id.is_empty() {
                                                        state.selected_vertebra_id.set(vertebra_id.clone());
                                                    }
                                                    state.studio_tab.set(*tab);
                                                    state.snapshot_current_novel(Some("/"));
                                                    nav("/", Default::default());
                                                }
                                                SearchTarget::Casting => nav("/casting", Default::default()),
                                                SearchTarget::Doc(path) => nav(path, Default::default()),
                                            }
                                        >"Open →"</button>
                                    </div>
                                </div>
                                <Show when=move || !h.excerpt.is_empty()>
                                    <p class="opacity-85">{h.excerpt.clone()}</p>
                                </Show>
                            </div>
                        }
                    }).collect_view().into_any()
                }}

                // Discovered Peer Network Hits
                <Show when=move || !gossip_hits.get().is_empty()>
                    <div class="pt-4 mt-4 border-t border-[#e5e5e0] dark:border-[#2d2d2a] space-y-2">
                        <div class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">"Discovered from Collaborator Network"</div>
                        {move || gossip_hits.get().into_iter().map(|item| {
                            let is_imported = imported_ids.get().contains(&item.id);
                            let item_imp = item.clone();
                            view! {
                                <div class="rounded-xl border border-[#5A5A40]/30 bg-[#f5f5f0]/60 dark:bg-[#252522]/60 p-3.5 space-y-1.5">
                                    <div class="flex flex-wrap items-start justify-between gap-2">
                                        <div>
                                            <span class="px-2 py-0.5 rounded bg-[#5A5A40] text-white text-[10px] mr-2">{format!("{} hop(s)", item.hop)}</span>
                                            <span class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">{item.title}</span>
                                            <div class="text-[11px] opacity-70">{format!("{} · {}", item.peer_label, item.peer_did)}</div>
                                        </div>
                                        <button
                                            type="button"
                                            disabled=is_imported
                                            class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white font-medium disabled:opacity-50"
                                            on:click=move |_| {
                                                imported_ids.update(|ids| if !ids.contains(&item_imp.id) { ids.push(item_imp.id); });
                                                if item_imp.is_novel {
                                                    let vid = format!("v-net-{}", state.novels.get().len() + 1);
                                                    state.novels.update(|list| {
                                                        list.push(NovelWorkspace {
                                                            id: format!("novel-net-{}", list.len() + 1),
                                                            title: item_imp.title.into(),
                                                            synopsis: item_imp.summary.into(),
                                                            collab_call: format!("Synced from {}", item_imp.peer_label),
                                                            seeking_collaborators: true,
                                                            namespace_hex: "peer..willow25".into(),
                                                            forking_allowed: true,
                                                            forking_locked: false,
                                                            favorited: true,
                                                            remixed_from_title: None,
                                                            remixed_baseline_text: None,
                                                            last_vertebra_id: vid.clone(),
                                                            last_studio_tab: StudioTab::Editor,
                                                            last_editor_view: EditorView::Split,
                                                            last_route: "/".into(),
                                                            pitches: Vec::new(),
                                                            gallery: Vec::new(),
                                                            vertebrae: vec![VertebraItem {
                                                                id: vid.clone(),
                                                                label: "Chapter · Peer Synced".into(),
                                                                kind: "Chapter".into(),
                                                                title: "Synced Chapter 1".into(),
                                                                storyline_date: item_imp.storyline_date.into(),
                                                                last_edited_ts: 500,
                                                                child_count: 1,
                                                                tags: item_imp.tags.iter().map(|s| (*s).to_string()).collect(),
                                                                content: format!("# {}\n\n{}", item_imp.title, item_imp.sample),
                                                                plot_blocks: vec![item_imp.summary.into()],
                                                            }],
                                                        });
                                                    });
                                                } else {
                                                    state.trunks.update(|list| {
                                                        list.push(CharacterTrunk {
                                                            id: format!("tr-net-{}", list.len() + 1),
                                                            name: item_imp.title.into(),
                                                            owner_name: item_imp.peer_label.into(),
                                                            owner_did: item_imp.peer_did.into(),
                                                            bio: item_imp.summary.into(),
                                                            sample: item_imp.sample.into(),
                                                            proposed_to: None,
                                                        });
                                                    });
                                                }
                                            }
                                        >{if is_imported { "Imported ✓" } else if item.is_novel { "Sync to Studio" } else { "Import to Casting" }}</button>
                                    </div>
                                    <p class="opacity-85">{format!("{} — {}", item.summary, item.sample)}</p>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn ProfilePage() -> impl IntoView {
    let state = expect_context::<AppState>();
    let name_edit = RwSignal::new(
        state
            .active_account
            .get_untracked()
            .map(|a| a.display_name)
            .unwrap_or_else(|| "Guest Author".into()),
    );
    let bio_edit = RwSignal::new(
        state
            .active_account
            .get_untracked()
            .map(|a| a.bio)
            .unwrap_or_default(),
    );
    let sec_edit = RwSignal::new(
        state
            .active_account
            .get_untracked()
            .map(|a| a.security_text)
            .unwrap_or_default(),
    );
    let avatar_edit = RwSignal::new(
        state
            .active_account
            .get_untracked()
            .map(|a| a.avatar_url)
            .unwrap_or_default(),
    );
    let saved_notice = RwSignal::new(String::new());
    let confirm_delete = RwSignal::new(false);

    view! {
        <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-4 text-xs max-w-2xl">
            <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">"Author Profile & did:peer:4 Identity"</h1>
            <Show
                when=move || state.active_account.get().is_some()
                fallback=|| view! { <p class="opacity-75">"No active identity unlocked. Visit Wallet Auth to create or import one."</p> }
            >
                <div class="space-y-3">
                    <div class="flex items-center gap-3">
                        {move || {
                            let url = avatar_edit.get();
                            let initials = state.active_account.get().map(|a| a.avatar_initials).unwrap_or_else(|| "AU".into());
                            if !url.trim().is_empty() {
                                view! {
                                    <img src=url alt="Author Avatar" class="w-12 h-12 rounded-full object-cover border border-[#e5e5e0] dark:border-[#3a3a35]" />
                                }.into_any()
                            } else {
                                view! {
                                    <div class="w-12 h-12 rounded-full bg-[#5A5A40] text-white flex items-center justify-center font-serif text-base font-bold">{initials}</div>
                                }.into_any()
                            }
                        }}
                        <div class="font-mono text-[11px] opacity-75">
                            {move || state.active_account.get().map(|a| a.did_short).unwrap_or_default()}
                        </div>
                    </div>
                    <label class="block">
                        <span class="font-medium">"Display Name"</span>
                        <input
                            type="text"
                            class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                            prop:value=move || name_edit.get()
                            on:input=move |ev| name_edit.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="block">
                        <span class="font-medium">"Avatar Image URL or Data URI"</span>
                        <input
                            type="text"
                            placeholder="https://... or data:image/..."
                            class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                            prop:value=move || avatar_edit.get()
                            on:input=move |ev| avatar_edit.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="block">
                        <span class="font-medium">"Author Bio"</span>
                        <textarea
                            rows="3"
                            class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                            prop:value=move || bio_edit.get()
                            on:input=move |ev| bio_edit.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="block">
                        <span class="font-medium">"Anti-Phishing Security Phrase"</span>
                        <input
                            type="text"
                            class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                            prop:value=move || sec_edit.get()
                            on:input=move |ev| sec_edit.set(event_target_value(&ev))
                        />
                    </label>
                    <div class="flex flex-wrap items-center justify-between gap-2 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a]">
                        <button
                            type="button"
                            class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium"
                            on:click=move |_| {
                                let updated_name = name_edit.get().trim().to_string();
                                let updated_bio = bio_edit.get().trim().to_string();
                                let updated_sec = sec_edit.get().trim().to_string();
                                let updated_avatar = avatar_edit.get().trim().to_string();
                                let active_did = state.active_account.get().map(|a| a.did_short);
                                state.active_account.update(|acc| {
                                    if let Some(a) = acc {
                                        if !updated_name.is_empty() {
                                            a.display_name = updated_name.clone();
                                            a.avatar_initials = updated_name.chars().take(2).collect::<String>().to_uppercase();
                                        }
                                        a.bio = updated_bio.clone();
                                        a.avatar_url = updated_avatar.clone();
                                        if !updated_sec.is_empty() { a.security_text = updated_sec.clone(); }
                                    }
                                });
                                if let Some(did) = active_did {
                                    state.accounts.update(|list| {
                                        if let Some(a) = list.iter_mut().find(|x| x.did_short == did) {
                                            if !updated_name.is_empty() {
                                                a.display_name = updated_name.clone();
                                                a.avatar_initials = updated_name.chars().take(2).collect::<String>().to_uppercase();
                                            }
                                            a.bio = updated_bio.clone();
                                            a.avatar_url = updated_avatar.clone();
                                            if !updated_sec.is_empty() { a.security_text = updated_sec.clone(); }
                                        }
                                    });
                                }
                                saved_notice.set("Profile saved.".into());
                            }
                        >"Save Profile"</button>

                        {move || if confirm_delete.get() {
                            view! {
                                <div class="flex gap-1.5">
                                    <button
                                        type="button"
                                        class="px-2.5 py-1.5 rounded-lg bg-red-700 text-white"
                                        on:click=move |_| {
                                            if let Some(did) = state.active_account.get().map(|a| a.did_short) {
                                                state.accounts.update(|list| list.retain(|a| a.did_short != did));
                                                state.active_account.set(state.accounts.get().first().cloned());
                                            }
                                            confirm_delete.set(false);
                                        }
                                    >"Confirm Delete Identity"</button>
                                    <button type="button" class="px-2.5 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| confirm_delete.set(false)>"Cancel"</button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <button type="button" class="px-2.5 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400" on:click=move |_| confirm_delete.set(true)>"Delete Identity"</button>
                            }.into_any()
                        }}
                    </div>
                    <Show when=move || !saved_notice.get().is_empty()>
                        <p class="text-[#5A5A40] dark:text-[#d0d0c8] font-medium">{move || saved_notice.get()}</p>
                    </Show>
                </div>
            </Show>
        </div>
    }
}
