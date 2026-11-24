use std::collections::HashMap;

use leptos::html;
use leptos::prelude::*;
use willow_gap_core::pio::{PioGate, PrivateInterest};

use crate::markdown::render_markdown_with_assets;
use crate::{
    AppState, ContactSubgroup, EditorView, GalleryAsset, SpineSortMode, StudioTab, VertebraItem,
};

#[component]
pub fn StudioPage() -> impl IntoView {
    let state = expect_context::<AppState>();
    let selected_id = state.selected_vertebra_id;
    let active_tab = state.studio_tab;
    let editor_view = state.editor_view;
    let split_container_ref = NodeRef::<html::Div>::new();
    let split_dragging = RwSignal::new(false);

    let new_vertebra_title = RwSignal::new(String::new());
    let confirm_delete_vertebra = RwSignal::new(false);
    let new_tag_text = RwSignal::new(String::new());
    let new_block_text = RwSignal::new(String::new());
    let editing_block_idx = RwSignal::new(Option::<usize>::None);
    let editing_block_buf = RwSignal::new(String::new());

    let new_subgroup_name = RwSignal::new(String::new());
    let editing_subgroup_idx = RwSignal::new(Option::<usize>::None);
    let editing_subgroup_buf = RwSignal::new(String::new());
    let new_contact_did = RwSignal::new(String::new());
    let selected_group_idx = RwSignal::new(0usize);
    let pio_note = RwSignal::new(String::new());

    let editing_novel_open = RwSignal::new(false);
    let edit_novel_title = RwSignal::new(String::new());
    let edit_novel_synopsis = RwSignal::new(String::new());
    let confirm_delete_novel = RwSignal::new(false);

    let image_modal_open = RwSignal::new(false);
    let new_image_title = RwSignal::new(String::new());
    let new_image_url = RwSignal::new(String::new());
    let editing_gallery_idx = RwSignal::new(Option::<usize>::None);
    let editing_gallery_title = RwSignal::new(String::new());
    let status_note = RwSignal::new(String::new());

    let sorted_vertebrae = move || {
        let novel_idx = state.active_novel_idx.get();
        let mut items = state
            .novels
            .get()
            .get(novel_idx)
            .map(|n| n.vertebrae.clone())
            .unwrap_or_default();
        match state.spine_sort.get() {
            SpineSortMode::Alphabetical => items.sort_by(|a, b| a.title.cmp(&b.title)),
            SpineSortMode::StorylineDate => {
                items.sort_by(|a, b| a.storyline_date.cmp(&b.storyline_date))
            }
            SpineSortMode::LastEdited => {
                items.sort_by(|a, b| b.last_edited_ts.cmp(&a.last_edited_ts))
            }
            SpineSortMode::Busyness => items.sort_by(|a, b| b.child_count.cmp(&a.child_count)),
        }
        items
    };

    let append_snippet = move |snippet: String| {
        let novel_idx = state.active_novel_idx.get();
        let target_id = selected_id.get();
        state.novels.update(|list| {
            if let Some(nv) = list.get_mut(novel_idx) {
                if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == target_id) {
                    if !v.content.is_empty() && !v.content.ends_with('\n') {
                        v.content.push('\n');
                    }
                    v.content.push_str(&snippet);
                    v.last_edited_ts += 1;
                }
            }
        });
    };

    let insert_asset_into_chapter = move |asset: GalleryAsset| {
        append_snippet(format!("![{}](internovel-img://{})\n", asset.title, asset.id));
        image_modal_open.set(false);
        active_tab.set(StudioTab::Editor);
        if editor_view.get() == EditorView::Edit {
            editor_view.set(EditorView::Split);
        }
        status_note.set(format!("Inserted '{}' into chapter.", asset.title));
    };

    let add_vertebra = move |_| {
        let title = new_vertebra_title.get().trim().to_string();
        if title.is_empty() {
            return;
        }
        let novel_idx = state.active_novel_idx.get();
        state.novels.update(|list| {
            if let Some(nv) = list.get_mut(novel_idx) {
                let next_id = format!("v-{}-{}", nv.vertebrae.len() + 1, title.len());
                nv.vertebrae.push(VertebraItem {
                    id: next_id.clone(),
                    label: "Idea · Draft".into(),
                    kind: "Idea".into(),
                    title: title.clone(),
                    storyline_date: "2026-01-01".into(),
                    last_edited_ts: 400,
                    child_count: 1,
                    tags: vec!["Scene".into()],
                    content: format!("# {title}\n\nNew section content."),
                    plot_blocks: vec![format!("Outline for {title}")],
                });
                nv.last_vertebra_id = next_id.clone();
                selected_id.set(next_id);
            }
        });
        new_vertebra_title.set(String::new());
    };

    let run_pio_probe = move |_| {
        let interest = PrivateInterest {
            namespace_id: [0x11; 32],
            subspace_id: None,
            path_prefix: vec![b"chapters".to_vec(), b"01".to_vec()],
        };
        let gate_a = PioGate::new([0xAA; 32], [0xBB; 32], vec![interest.clone()], 64);
        let gate_b = PioGate::new([0xBB; 32], [0xAA; 32], vec![interest], 64);
        let digests = gate_a.advertised_digests();
        let overlaps = gate_b.confirm_overlaps(&digests);
        pio_note.set(format!(
            "Checked {} private interest digests; {} mutual sync overlap(s) confirmed.",
            digests.len(),
            overlaps.len()
        ));
    };

    view! {
        {move || {
            if state.novels.get().is_empty() {
                return view! {
                    <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-8 text-center text-xs max-w-lg mx-auto my-8 space-y-2">
                        <h2 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">"No Novels in Workspace"</h2>
                        <p class="opacity-75">"Use '+ Novel' in the top bar to start a new novel workspace."</p>
                    </div>
                }.into_any();
            }

            view! {
        <div class="flex flex-col lg:flex-row gap-4 items-start">
            // Collapsible Left Navigation: Novel Spine
            {move || if state.spine_collapsed.get() {
                let count = sorted_vertebrae().len();
                view! {
                    <aside class="w-full lg:w-11 shrink-0 rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-2 flex lg:flex-col items-center justify-between lg:justify-start gap-3">
                        <button
                            type="button"
                            title="Expand Novel Spine"
                            class="w-7 h-7 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] flex items-center justify-center text-xs font-bold"
                            on:click=move |_| state.spine_collapsed.set(false)
                        >"»"</button>
                        <span class="font-serif text-[11px] font-bold uppercase tracking-widest text-[#5A5A40] dark:text-[#d0d0c8] lg:[writing-mode:vertical-rl] lg:rotate-180">
                            {format!("Novel Spine ({count})")}
                        </span>
                    </aside>
                }.into_any()
            } else {
                let novel_idx_s = state.active_novel_idx.get();
                let current_novel = state.novels.get().get(novel_idx_s).cloned();
                view! {
                    <aside class="w-full lg:w-68 shrink-0 rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-3 space-y-3">
                        <div class="flex items-center justify-between gap-1.5">
                            <h2 class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">
                                {move || if state.spine_view.get() { "Novel Spine" } else { "Flat List" }}
                            </h2>
                            <div class="flex items-center gap-1">
                                <button
                                    type="button"
                                    class="text-[11px] px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                    on:click=move |_| state.spine_view.update(|v| *v = !*v)
                                >
                                    {move || if state.spine_view.get() { "Flat" } else { "Spine" }}
                                </button>
                                <button
                                    type="button"
                                    title="Collapse Novel Spine"
                                    class="text-[11px] px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                    on:click=move |_| state.spine_collapsed.set(true)
                                >"«"</button>
                            </div>
                        </div>

                        <select
                            aria-label="Sort Spine"
                            class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1 text-xs"
                            on:change=move |ev| {
                                match event_target_value(&ev).as_str() {
                                    "alpha" => state.spine_sort.set(SpineSortMode::Alphabetical),
                                    "date" => state.spine_sort.set(SpineSortMode::StorylineDate),
                                    "edited" => state.spine_sort.set(SpineSortMode::LastEdited),
                                    "busy" => state.spine_sort.set(SpineSortMode::Busyness),
                                    _ => {}
                                }
                            }
                        >
                            <option value="date">"Sort: Storyline Date"</option>
                            <option value="alpha">"Sort: Alphabetical"</option>
                            <option value="edited">"Sort: Last Edited"</option>
                            <option value="busy">"Sort: Busyness"</option>
                        </select>

                        <div class="space-y-1">
                            {move || sorted_vertebrae().into_iter().map(|item| {
                                let id = item.id.clone();
                                let is_selected = selected_id.get() == id;
                                view! {
                                    <button
                                        type="button"
                                        class=if is_selected {
                                            "w-full text-left rounded-lg px-2.5 py-2 text-xs bg-[#5A5A40] text-white"
                                        } else {
                                            "w-full text-left rounded-lg px-2.5 py-2 text-xs hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]"
                                        }
                                        on:click=move |_| {
                                            confirm_delete_vertebra.set(false);
                                            selected_id.set(id.clone());
                                            state.snapshot_current_novel(Some("/"));
                                        }
                                    >
                                        <Show when=move || state.spine_view.get()>
                                            <div class="text-[10px] uppercase tracking-wide opacity-75">
                                                {format!("{} · {}", item.kind, item.storyline_date)}
                                            </div>
                                        </Show>
                                        <div class="font-medium">{item.title}</div>
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        <div class="flex gap-1 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a]">
                            <input
                                type="text"
                                placeholder="Add Vertebra / Chapter..."
                                class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1 text-xs"
                                prop:value=move || new_vertebra_title.get()
                                on:input=move |ev| new_vertebra_title.set(event_target_value(&ev))
                            />
                            <button type="button" class="px-2.5 py-1 rounded-lg bg-[#5A5A40] text-white text-xs" on:click=add_vertebra>"+"</button>
                        </div>

                        {current_novel.map(|nv| {
                            let nv_title = nv.title.clone();
                            let nv_syn = nv.synopsis.clone();
                            view! {
                                <div class="pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] space-y-2 text-[11px]">
                                    <div class="flex items-center justify-between gap-1">
                                        <label class="inline-flex items-center gap-1.5 cursor-pointer">
                                            <input
                                                type="checkbox"
                                                prop:checked=nv.forking_allowed
                                                disabled=nv.forking_locked
                                                on:change=move |ev| {
                                                    let checked = event_target_checked(&ev);
                                                    state.novels.update(|list| {
                                                        if let Some(target) = list.get_mut(novel_idx_s) {
                                                            if !target.forking_locked { target.forking_allowed = checked; }
                                                        }
                                                    });
                                                }
                                            />
                                            <span>{if nv.forking_locked { "Forking Locked" } else { "Allow Forking" }}</span>
                                        </label>
                                        <button
                                            type="button"
                                            class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                            on:click=move |_| {
                                                let opening = !editing_novel_open.get();
                                                if opening {
                                                    edit_novel_title.set(nv_title.clone());
                                                    edit_novel_synopsis.set(nv_syn.clone());
                                                    confirm_delete_novel.set(false);
                                                }
                                                editing_novel_open.set(opening);
                                            }
                                        >{move || if editing_novel_open.get() { "Done" } else { "Novel Details" }}</button>
                                    </div>

                                    <Show when=move || editing_novel_open.get()>
                                        <div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-2.5 space-y-2">
                                            <input
                                                type="text"
                                                class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-2 py-1 text-xs font-medium"
                                                prop:value=move || edit_novel_title.get()
                                                on:input=move |ev| edit_novel_title.set(event_target_value(&ev))
                                            />
                                            <textarea
                                                rows="2"
                                                placeholder="Novel synopsis..."
                                                class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-2 py-1 text-xs"
                                                prop:value=move || edit_novel_synopsis.get()
                                                on:input=move |ev| edit_novel_synopsis.set(event_target_value(&ev))
                                            />
                                            <div class="flex items-center justify-between gap-1">
                                                <button
                                                    type="button"
                                                    class="px-2.5 py-1 rounded bg-[#5A5A40] text-white"
                                                    on:click=move |_| {
                                                        let t = edit_novel_title.get().trim().to_string();
                                                        let s = edit_novel_synopsis.get().trim().to_string();
                                                        state.novels.update(|list| {
                                                            if let Some(target) = list.get_mut(novel_idx_s) {
                                                                if !t.is_empty() { target.title = t; }
                                                                target.synopsis = s;
                                                            }
                                                        });
                                                        editing_novel_open.set(false);
                                                    }
                                                >"Save"</button>
                                                {move || if confirm_delete_novel.get() {
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class="px-2 py-1 rounded bg-red-700 text-white"
                                                            on:click=move |_| {
                                                                state.novels.update(|list| {
                                                                    if novel_idx_s < list.len() { list.remove(novel_idx_s); }
                                                                });
                                                                let len = state.novels.get().len();
                                                                let next_idx = if len == 0 { 0 } else { novel_idx_s.min(len - 1) };
                                                                state.active_novel_idx.set(next_idx);
                                                                if let Some(first) = state.novels.get().get(next_idx).and_then(|n| n.vertebrae.first()) {
                                                                    selected_id.set(first.id.clone());
                                                                }
                                                                editing_novel_open.set(false);
                                                                confirm_delete_novel.set(false);
                                                            }
                                                        >"Confirm Delete"</button>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400"
                                                            on:click=move |_| confirm_delete_novel.set(true)
                                                        >"Delete Novel"</button>
                                                    }.into_any()
                                                }}
                                            </div>
                                        </div>
                                    </Show>
                                </div>
                            }
                        })}
                    </aside>
                }.into_any()
            }}

            // Main Studio Workspace
            <section class="flex-1 min-w-0 w-full rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-4">
                <div class="flex flex-wrap items-center justify-between gap-2 border-b border-[#e5e5e0] dark:border-[#2d2d2a] pb-3">
                    <div class="flex flex-wrap gap-1 text-xs">
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| { active_tab.set(StudioTab::Editor); state.snapshot_current_novel(Some("/")); }>"Novel Editor"</button>
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| { active_tab.set(StudioTab::PlotPlanner); state.snapshot_current_novel(Some("/")); }>"Plot Planner"</button>
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| { active_tab.set(StudioTab::ContactsSync); state.snapshot_current_novel(Some("/")); }>"Contacts & Collaborators"</button>
                        <button type="button" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| { active_tab.set(StudioTab::Gallery); state.snapshot_current_novel(Some("/")); }>
                            {move || {
                                let cnt = state.novels.get().get(state.active_novel_idx.get()).map(|n| n.gallery.len()).unwrap_or(0);
                                format!("Image Gallery ({cnt})")
                            }}
                        </button>
                    </div>
                    <span class="text-[11px] opacity-70">"Vertebrae metadata excluded when publishing"</span>
                </div>

                <Show when=move || !status_note.get().is_empty()>
                    <div class="rounded-lg bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5 text-xs flex items-center justify-between">
                        <span>{move || status_note.get()}</span>
                        <button type="button" on:click=move |_| status_note.set(String::new())>"×"</button>
                    </div>
                </Show>

                {move || match active_tab.get() {
                    StudioTab::Editor => {
                        let novel_idx = state.active_novel_idx.get();
                        let current_vertebra = move || {
                            let tid = selected_id.get();
                            state.novels.get()
                                .get(novel_idx)
                                .and_then(|nv| nv.vertebrae.iter().find(|v| v.id == tid).cloned())
                        };
                        let current_content = move || current_vertebra().map(|v| v.content).unwrap_or_default();
                        let current_assets = move || {
                            state.novels.get()
                                .get(novel_idx)
                                .map(|nv| {
                                    nv.gallery.iter().map(|g| (g.id.clone(), g.data_url.clone())).collect::<HashMap<_, _>>()
                                })
                                .unwrap_or_default()
                        };

                        view! {
                            <div class="space-y-3">
                                // Chapter/Vertebra Metadata Editor + Confirm Delete
                                <div class="flex flex-wrap items-center justify-between gap-2 pb-2 border-b border-[#e5e5e0] dark:border-[#2d2d2a] text-xs">
                                    <div class="flex flex-wrap items-center gap-2">
                                        <input
                                            type="text"
                                            class="font-serif font-bold text-sm rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1"
                                            prop:value=move || current_vertebra().map(|v| v.title).unwrap_or_default()
                                            on:change=move |ev| {
                                                let val = event_target_value(&ev);
                                                let tid = selected_id.get();
                                                state.novels.update(|list| {
                                                    if let Some(nv) = list.get_mut(novel_idx) {
                                                        if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid) {
                                                            v.title = val;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <select
                                            class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1"
                                            on:change=move |ev| {
                                                let val = event_target_value(&ev);
                                                let tid = selected_id.get();
                                                state.novels.update(|list| {
                                                    if let Some(nv) = list.get_mut(novel_idx) {
                                                        if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid) {
                                                            v.kind = val;
                                                        }
                                                    }
                                                });
                                            }
                                        >
                                            <option value="Idea">"Idea"</option>
                                            <option value="Plot Point">"Plot Point"</option>
                                            <option value="Theme">"Theme"</option>
                                            <option value="Chapter">"Chapter"</option>
                                        </select>
                                        <input
                                            type="date"
                                            class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1"
                                            prop:value=move || current_vertebra().map(|v| v.storyline_date).unwrap_or_default()
                                            on:change=move |ev| {
                                                let val = event_target_value(&ev);
                                                let tid = selected_id.get();
                                                state.novels.update(|list| {
                                                    if let Some(nv) = list.get_mut(novel_idx) {
                                                        if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid) {
                                                            v.storyline_date = val;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                    </div>
                                    <div class="flex items-center gap-1.5">
                                        {move || if confirm_delete_vertebra.get() {
                                            view! {
                                                <button
                                                    type="button"
                                                    class="px-2 py-1 rounded bg-red-700 text-white"
                                                    on:click=move |_| {
                                                        let tid = selected_id.get();
                                                        state.novels.update(|list| {
                                                            if let Some(nv) = list.get_mut(novel_idx) {
                                                                if nv.vertebrae.len() > 1 {
                                                                    nv.vertebrae.retain(|x| x.id != tid);
                                                                    if let Some(first) = nv.vertebrae.first() {
                                                                        nv.last_vertebra_id = first.id.clone();
                                                                        selected_id.set(first.id.clone());
                                                                    }
                                                                }
                                                            }
                                                        });
                                                        confirm_delete_vertebra.set(false);
                                                    }
                                                >"Confirm"</button>
                                                <button
                                                    type="button"
                                                    class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                    on:click=move |_| confirm_delete_vertebra.set(false)
                                                >"Cancel"</button>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <button
                                                    type="button"
                                                    class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400"
                                                    on:click=move |_| confirm_delete_vertebra.set(true)
                                                >"Delete Chapter"</button>
                                            }.into_any()
                                        }}
                                    </div>
                                </div>

                                // Tags CRUD Strip
                                <div class="flex flex-wrap items-center gap-1.5 text-xs">
                                    {move || current_vertebra().map(|v| v.tags).unwrap_or_default().into_iter().map(|tag| {
                                        let t_rm = tag.clone();
                                        view! {
                                            <span class="inline-flex items-center gap-1 rounded-full bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-0.5 text-[11px]">
                                                {tag}
                                                <button
                                                    type="button"
                                                    on:click=move |_| {
                                                        let tid = selected_id.get();
                                                        state.novels.update(|list| {
                                                            if let Some(nv) = list.get_mut(novel_idx) {
                                                                if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid) {
                                                                    v.tags.retain(|t| t != &t_rm);
                                                                }
                                                            }
                                                        });
                                                    }
                                                >"×"</button>
                                            </span>
                                        }
                                    }).collect_view()}
                                    <input
                                        type="text"
                                        placeholder="Add tag (e.g. Character:Lyra)..."
                                        class="rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5 text-[11px]"
                                        prop:value=move || new_tag_text.get()
                                        on:input=move |ev| new_tag_text.set(event_target_value(&ev))
                                    />
                                    <button
                                        type="button"
                                        class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-[11px]"
                                        on:click=move |_| {
                                            let tag = new_tag_text.get().trim().to_string();
                                            if tag.is_empty() { return; }
                                            let tid = selected_id.get();
                                            state.novels.update(|list| {
                                                if let Some(nv) = list.get_mut(novel_idx) {
                                                    if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid) {
                                                        if !v.tags.contains(&tag) { v.tags.push(tag); }
                                                    }
                                                }
                                            });
                                            new_tag_text.set(String::new());
                                        }
                                    >"+ Tag"</button>
                                </div>

                                // Editor Toolbar
                                <div class="flex flex-wrap items-center justify-between gap-2">
                                    <div class="flex flex-wrap gap-1 text-xs">
                                        <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| append_snippet("# Heading\n".into())>"H1"</button>
                                        <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| append_snippet("## Section\n".into())>"H2"</button>
                                        <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| append_snippet("**bold**".into())>"Bold"</button>
                                        <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| append_snippet("*italic*".into())>"Italic"</button>
                                        <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| append_snippet("> Quote\n".into())>"Quote"</button>
                                        <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| append_snippet("- Item\n".into())>"List"</button>
                                        <button type="button" class="px-2.5 py-1 rounded border border-[#5A5A40]/40 bg-[#f5f5f0] dark:bg-[#252522] font-medium" on:click=move |_| image_modal_open.set(true)>"+ Image"</button>
                                    </div>
                                    <div class="flex items-center gap-2 text-xs">
                                        <Show when=move || editor_view.get() == EditorView::Split>
                                            <span class="hidden md:inline font-mono text-[11px] opacity-65">
                                                {move || format!("{:.0}% / {:.0}%", state.split_ratio.get(), 100.0 - state.split_ratio.get())}
                                            </span>
                                        </Show>
                                        <div class="flex gap-1">
                                            <button type="button" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| { editor_view.set(EditorView::Edit); state.snapshot_current_novel(Some("/")); }>"Edit"</button>
                                            <button type="button" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| { editor_view.set(EditorView::Split); state.snapshot_current_novel(Some("/")); }>"Split"</button>
                                            <button type="button" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| { editor_view.set(EditorView::Preview); state.snapshot_current_novel(Some("/")); }>"Preview"</button>
                                        </div>
                                    </div>
                                </div>

                                // Split / Edit / Preview with Stateful Sliding Divider
                                <div
                                    node_ref=split_container_ref
                                    class=move || if editor_view.get() == EditorView::Split { "flex flex-col md:flex-row gap-3 md:gap-0 items-stretch min-h-[420px]" } else { "block" }
                                    on:pointermove=move |ev| {
                                        if !split_dragging.get() { return; }
                                        if let Some(container) = split_container_ref.get() {
                                            let rect = container.get_bounding_client_rect();
                                            if rect.width() > 0.0 {
                                                let pct = ((ev.client_x() as f64 - rect.left()) / rect.width()) * 100.0;
                                                state.split_ratio.set(pct.clamp(20.0, 80.0));
                                            }
                                        }
                                    }
                                    on:pointerup=move |_| split_dragging.set(false)
                                    on:pointercancel=move |_| split_dragging.set(false)
                                >
                                    <Show when=move || editor_view.get() != EditorView::Preview>
                                        <div
                                            class=move || if editor_view.get() == EditorView::Split { "w-full md:shrink-0 flex flex-col min-w-0" } else { "w-full" }
                                            style=move || if editor_view.get() == EditorView::Split { format!("width: {}%;", state.split_ratio.get()) } else { String::new() }
                                        >
                                            <textarea
                                                rows="18"
                                                class="w-full flex-1 min-h-[400px] rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-3 font-serif text-sm leading-relaxed"
                                                prop:value=current_content
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    let tid = selected_id.get();
                                                    state.novels.update(|list| {
                                                        if let Some(nv) = list.get_mut(novel_idx) {
                                                            if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid) {
                                                                v.content = val;
                                                            }
                                                        }
                                                    });
                                                }
                                            />
                                        </div>
                                    </Show>

                                    <Show when=move || editor_view.get() == EditorView::Split>
                                        <div
                                            title="Drag to resize · Double-click to reset 50/50"
                                            class="hidden md:flex w-3.5 shrink-0 cursor-col-resize items-center justify-center group select-none"
                                            on:pointerdown=move |ev| {
                                                ev.prevent_default();
                                                split_dragging.set(true);
                                            }
                                            on:dblclick=move |_| state.split_ratio.set(50.0)
                                        >
                                            <div class="w-1 h-12 rounded-full bg-[#e5e5e0] dark:bg-[#3a3a35] group-hover:bg-[#5A5A40]"></div>
                                        </div>
                                    </Show>

                                    <Show when=move || editor_view.get() != EditorView::Edit>
                                        <article
                                            class="flex-1 min-w-0 prose prose-stone dark:prose-invert prose-micro max-w-none rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-4 min-h-[400px] overflow-y-auto"
                                            style=move || format!("font-size: {}%;", state.text_scale.get())
                                            inner_html=move || render_markdown_with_assets(&current_content(), &current_assets())
                                        />
                                    </Show>
                                </div>
                            </div>
                        }.into_any()
                    }
                    StudioTab::PlotPlanner => {
                        let novel_idx = state.active_novel_idx.get();
                        let tid = selected_id.get();
                        let blocks = state.novels.get()
                            .get(novel_idx)
                            .and_then(|nv| nv.vertebrae.iter().find(|v| v.id == tid))
                            .map(|v| v.plot_blocks.clone())
                            .unwrap_or_default();

                        view! {
                            <div class="space-y-3 text-xs">
                                <p class="opacity-75">"Block-based Plot Planner attached to this Vertebra."</p>
                                <div class="space-y-2">
                                    {blocks.into_iter().enumerate().map(|(i, b)| {
                                        let tid_up = selected_id.get();
                                        let tid_dn = selected_id.get();
                                        let tid_del = selected_id.get();
                                        let tid_save = selected_id.get();
                                        let b_edit = b.clone();
                                        view! {
                                            <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-3 flex items-center justify-between gap-2">
                                                {move || if editing_block_idx.get() == Some(i) {
                                                    let t_s = tid_save.clone();
                                                    view! {
                                                        <div class="flex-1 flex gap-2">
                                                            <input
                                                                type="text"
                                                                class="flex-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-2 py-1"
                                                                prop:value=move || editing_block_buf.get()
                                                                on:input=move |ev| editing_block_buf.set(event_target_value(&ev))
                                                            />
                                                            <button
                                                                type="button"
                                                                class="px-2 py-1 rounded bg-[#5A5A40] text-white"
                                                                on:click=move |_| {
                                                                    let updated = editing_block_buf.get().trim().to_string();
                                                                    if !updated.is_empty() {
                                                                        state.novels.update(|list| {
                                                                            if let Some(nv) = list.get_mut(novel_idx) {
                                                                                if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == t_s) {
                                                                                    if let Some(slot) = v.plot_blocks.get_mut(i) {
                                                                                        *slot = updated;
                                                                                    }
                                                                                }
                                                                            }
                                                                        });
                                                                    }
                                                                    editing_block_idx.set(None);
                                                                }
                                                            >"Save"</button>
                                                            <button
                                                                type="button"
                                                                class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                                on:click=move |_| editing_block_idx.set(None)
                                                            >"Cancel"</button>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    let b_init = b_edit.clone();
                                                    view! {
                                                        <span class="flex-1">{format!("{}. {}", i + 1, b)}</span>
                                                        <button
                                                            type="button"
                                                            class="px-1.5 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                            on:click=move |_| {
                                                                editing_block_buf.set(b_init.clone());
                                                                editing_block_idx.set(Some(i));
                                                            }
                                                        >"Edit"</button>
                                                    }.into_any()
                                                }}
                                                <div class="flex gap-1">
                                                    <button
                                                        type="button"
                                                        class="px-1.5 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                        on:click=move |_| {
                                                            if i == 0 { return; }
                                                            state.novels.update(|list| {
                                                                if let Some(nv) = list.get_mut(novel_idx) {
                                                                    if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid_up) {
                                                                        if i < v.plot_blocks.len() { v.plot_blocks.swap(i - 1, i); }
                                                                    }
                                                                }
                                                            });
                                                        }
                                                    >"↑"</button>
                                                    <button
                                                        type="button"
                                                        class="px-1.5 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                        on:click=move |_| {
                                                            state.novels.update(|list| {
                                                                if let Some(nv) = list.get_mut(novel_idx) {
                                                                    if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid_dn) {
                                                                        if i + 1 < v.plot_blocks.len() { v.plot_blocks.swap(i, i + 1); }
                                                                    }
                                                                }
                                                            });
                                                        }
                                                    >"↓"</button>
                                                    <button
                                                        type="button"
                                                        class="px-1.5 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400"
                                                        on:click=move |_| {
                                                            state.novels.update(|list| {
                                                                if let Some(nv) = list.get_mut(novel_idx) {
                                                                    if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid_del) {
                                                                        if i < v.plot_blocks.len() { v.plot_blocks.remove(i); }
                                                                    }
                                                                }
                                                            });
                                                        }
                                                    >"×"</button>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                                <div class="flex gap-2">
                                    <input
                                        type="text"
                                        placeholder="Add beat or plot card..."
                                        class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                                        prop:value=move || new_block_text.get()
                                        on:input=move |ev| new_block_text.set(event_target_value(&ev))
                                    />
                                    <button
                                        type="button"
                                        class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white"
                                        on:click=move |_| {
                                            let txt = new_block_text.get().trim().to_string();
                                            if txt.is_empty() { return; }
                                            let tid = selected_id.get();
                                            state.novels.update(|list| {
                                                if let Some(nv) = list.get_mut(novel_idx) {
                                                    if let Some(v) = nv.vertebrae.iter_mut().find(|x| x.id == tid) {
                                                        v.plot_blocks.push(txt);
                                                        v.child_count = v.plot_blocks.len();
                                                    }
                                                }
                                            });
                                            new_block_text.set(String::new());
                                        }
                                    >"Add Block"</button>
                                </div>
                            </div>
                        }.into_any()
                    }
                    StudioTab::ContactsSync => {
                        view! {
                            <div class="space-y-4 text-xs">
                                <div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-3 space-y-2">
                                    <div class="font-semibold text-[#5A5A40] dark:text-[#d0d0c8]">"Private Collaborator Sync"</div>
                                    <button type="button" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white" on:click=run_pio_probe>
                                        "Check Sync Overlap"
                                    </button>
                                    <Show when=move || !pio_note.get().is_empty()>
                                        <p>{move || pio_note.get()}</p>
                                    </Show>
                                </div>

                                <div class="space-y-2">
                                    <div class="font-semibold">"Contacts & Permissions Subgroups"</div>
                                    <div class="flex gap-2">
                                        <input
                                            type="text"
                                            placeholder="Create new subgroup (e.g. Illustrators)..."
                                            class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                                            prop:value=move || new_subgroup_name.get()
                                            on:input=move |ev| new_subgroup_name.set(event_target_value(&ev))
                                        />
                                        <button
                                            type="button"
                                            class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                            on:click=move |_| {
                                                let name = new_subgroup_name.get().trim().to_string();
                                                if name.is_empty() { return; }
                                                state.subgroups.update(|groups| {
                                                    if !groups.iter().any(|g| g.name.eq_ignore_ascii_case(&name)) {
                                                        groups.push(ContactSubgroup { name, member_dids: Vec::new() });
                                                    }
                                                });
                                                new_subgroup_name.set(String::new());
                                            }
                                        >"+ Subgroup"</button>
                                    </div>
                                    {move || state.subgroups.get().into_iter().enumerate().map(|(g_idx, grp)| {
                                        let gname = grp.name.clone();
                                        let gname_btn = grp.name.clone();
                                        let gname_del = grp.name.clone();
                                        let gname_edit = grp.name.clone();
                                        view! {
                                            <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-3 space-y-1.5">
                                                <div class="flex items-center justify-between gap-2">
                                                    {move || if editing_subgroup_idx.get() == Some(g_idx) {
                                                        view! {
                                                            <div class="flex flex-1 gap-1.5">
                                                                <input
                                                                    type="text"
                                                                    class="flex-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5"
                                                                    prop:value=move || editing_subgroup_buf.get()
                                                                    on:input=move |ev| editing_subgroup_buf.set(event_target_value(&ev))
                                                                />
                                                                <button
                                                                    type="button"
                                                                    class="px-2 py-0.5 rounded bg-[#5A5A40] text-white"
                                                                    on:click=move |_| {
                                                                        let updated = editing_subgroup_buf.get().trim().to_string();
                                                                        if !updated.is_empty() {
                                                                            state.subgroups.update(|groups| {
                                                                                if let Some(slot) = groups.get_mut(g_idx) {
                                                                                    slot.name = updated;
                                                                                }
                                                                            });
                                                                        }
                                                                        editing_subgroup_idx.set(None);
                                                                    }
                                                                >"Save"</button>
                                                                <button type="button" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| editing_subgroup_idx.set(None)>"Cancel"</button>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        let gn_display = gname.clone();
                                                        let gn_edit_init = gname_edit.clone();
                                                        let gn_del = gname_del.clone();
                                                        view! {
                                                            <span class="font-medium">{gn_display}</span>
                                                            <div class="flex gap-1.5">
                                                                <button
                                                                    type="button"
                                                                    class="text-[11px] px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]"
                                                                    on:click=move |_| {
                                                                        editing_subgroup_buf.set(gn_edit_init.clone());
                                                                        editing_subgroup_idx.set(Some(g_idx));
                                                                    }
                                                                >"Rename"</button>
                                                                <button
                                                                    type="button"
                                                                    class="text-[11px] text-red-700 dark:text-red-400"
                                                                    on:click=move |_| {
                                                                        state.subgroups.update(|groups| groups.retain(|g| g.name != gn_del));
                                                                    }
                                                                >"Delete Subgroup"</button>
                                                            </div>
                                                        }.into_any()
                                                    }}
                                                </div>
                                                <div class="flex flex-wrap gap-1.5">
                                                    {if grp.member_dids.is_empty() {
                                                        view! { <span class="text-[11px] opacity-65">"No members yet"</span> }.into_any()
                                                    } else {
                                                        grp.member_dids.into_iter().map(|did| {
                                                            let did_rm = did.clone();
                                                            let g_rm = gname_btn.clone();
                                                            view! {
                                                                <span class="inline-flex items-center gap-1 rounded bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5 font-mono text-[11px]">
                                                                    {did}
                                                                    <button
                                                                        type="button"
                                                                        class="text-red-700 dark:text-red-400"
                                                                        title="Remove & revoke access"
                                                                        on:click=move |_| {
                                                                            state.subgroups.update(|groups| {
                                                                                if let Some(g) = groups.iter_mut().find(|x| x.name == g_rm) {
                                                                                    g.member_dids.retain(|m| m != &did_rm);
                                                                                }
                                                                            });
                                                                        }
                                                                    >"×"</button>
                                                                </span>
                                                            }
                                                        }).collect_view().into_any()
                                                    }}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                    <div class="flex flex-wrap gap-2">
                                        <input
                                            type="text"
                                            placeholder="did:peer:4zQm... collaborator DID"
                                            class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5"
                                            prop:value=move || new_contact_did.get()
                                            on:input=move |ev| new_contact_did.set(event_target_value(&ev))
                                        />
                                        <select
                                            class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5"
                                            on:change=move |ev| {
                                                if let Ok(idx) = event_target_value(&ev).parse::<usize>() {
                                                    selected_group_idx.set(idx);
                                                }
                                            }
                                        >
                                            {move || state.subgroups.get().into_iter().enumerate().map(|(i, g)| {
                                                view! { <option value=i.to_string()>{g.name}</option> }
                                            }).collect_view()}
                                        </select>
                                        <button
                                            type="button"
                                            class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white"
                                            on:click=move |_| {
                                                let d = new_contact_did.get().trim().to_string();
                                                if d.is_empty() { return; }
                                                let idx = selected_group_idx.get();
                                                state.subgroups.update(|groups| {
                                                    if let Some(target) = groups.get_mut(idx) {
                                                        target.member_dids.push(d);
                                                    }
                                                });
                                                new_contact_did.set(String::new());
                                            }
                                        >"Add Collaborator"</button>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                    StudioTab::Gallery => {
                        let novel_idx = state.active_novel_idx.get();
                        let assets = state.novels.get().get(novel_idx).map(|n| n.gallery.clone()).unwrap_or_default();
                        view! {
                            <div class="space-y-4 text-xs">
                                <div class="rounded-2xl border-2 border-dashed border-[#5A5A40]/40 bg-[#f5f5f0] dark:bg-[#252522] p-5 space-y-2">
                                    <div class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">"Add Image to Novel Gallery"</div>
                                    <div class="grid gap-2 md:grid-cols-3">
                                        <input
                                            type="text"
                                            placeholder="Caption / Title"
                                            class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-3 py-1.5"
                                            prop:value=move || new_image_title.get()
                                            on:input=move |ev| new_image_title.set(event_target_value(&ev))
                                        />
                                        <input
                                            type="text"
                                            placeholder="Image Data URL or asset URI..."
                                            class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-3 py-1.5"
                                            prop:value=move || new_image_url.get()
                                            on:input=move |ev| new_image_url.set(event_target_value(&ev))
                                        />
                                        <button
                                            type="button"
                                            class="rounded-lg bg-[#5A5A40] text-white px-3 py-1.5 font-medium"
                                            on:click=move |_| {
                                                let url = new_image_url.get().trim().to_string();
                                                if url.is_empty() { return; }
                                                let title = new_image_title.get().trim().to_string();
                                                let label = if title.is_empty() { "Scene Illustration".into() } else { title };
                                                state.novels.update(|list| {
                                                    if let Some(nv) = list.get_mut(novel_idx) {
                                                        nv.gallery.push(GalleryAsset {
                                                            id: format!("img-{}", nv.gallery.len() + 1),
                                                            title: label,
                                                            data_url: url,
                                                            width: 1024,
                                                            height: 768,
                                                            size_kb: 120,
                                                        });
                                                    }
                                                });
                                                new_image_title.set(String::new());
                                                new_image_url.set(String::new());
                                            }
                                        >"Save to Gallery"</button>
                                    </div>
                                </div>
                                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                                    {assets.into_iter().enumerate().map(|(idx, asset)| {
                                        let a_ins = asset.clone();
                                        let title_init = asset.title.clone();
                                        let title_view = asset.title.clone();
                                        view! {
                                            <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-3 space-y-2">
                                                <img src=asset.data_url.clone() alt=asset.title.clone() class="w-full h-36 object-cover rounded-lg bg-[#f5f5f0] dark:bg-[#252522]"/>
                                                {move || if editing_gallery_idx.get() == Some(idx) {
                                                    view! {
                                                        <div class="flex gap-1">
                                                            <input
                                                                type="text"
                                                                class="flex-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5 text-[11px]"
                                                                prop:value=move || editing_gallery_title.get()
                                                                on:input=move |ev| editing_gallery_title.set(event_target_value(&ev))
                                                            />
                                                            <button
                                                                type="button"
                                                                class="px-2 py-0.5 rounded bg-[#5A5A40] text-white"
                                                                on:click=move |_| {
                                                                    let updated = editing_gallery_title.get().trim().to_string();
                                                                    if !updated.is_empty() {
                                                                        state.novels.update(|list| {
                                                                            if let Some(nv) = list.get_mut(novel_idx) {
                                                                                if let Some(slot) = nv.gallery.get_mut(idx) { slot.title = updated; }
                                                                            }
                                                                        });
                                                                    }
                                                                    editing_gallery_idx.set(None);
                                                                }
                                                            >"Save"</button>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    let t_show = title_view.clone();
                                                    let t_set = title_init.clone();
                                                    view! {
                                                        <div class="flex items-center justify-between gap-1">
                                                            <div class="font-medium truncate">{t_show}</div>
                                                            <button
                                                                type="button"
                                                                class="text-[11px] opacity-70 hover:opacity-100"
                                                                on:click=move |_| {
                                                                    editing_gallery_title.set(t_set.clone());
                                                                    editing_gallery_idx.set(Some(idx));
                                                                }
                                                            >"Rename"</button>
                                                        </div>
                                                    }.into_any()
                                                }}
                                                <div class="flex flex-wrap gap-1.5">
                                                    <button type="button" class="px-2.5 py-1 rounded bg-[#5A5A40] text-white" on:click=move |_| insert_asset_into_chapter(a_ins.clone())>"Insert into Chapter"</button>
                                                    <button
                                                        type="button"
                                                        class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400"
                                                        on:click=move |_| {
                                                            state.novels.update(|list| {
                                                                if let Some(nv) = list.get_mut(novel_idx) {
                                                                    if idx < nv.gallery.len() { nv.gallery.remove(idx); }
                                                                }
                                                            });
                                                            editing_gallery_idx.set(None);
                                                        }
                                                    >"Delete"</button>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_any()
                    }
                }}
            </section>
        </div>

        <Show when=move || image_modal_open.get()>
            <div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4">
                <div class="w-full max-w-2xl rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-4 text-xs">
                    <div class="flex items-center justify-between border-b border-[#e5e5e0] dark:border-[#2d2d2a] pb-2">
                        <h3 class="font-serif text-base font-bold text-[#5A5A40] dark:text-[#e6e6de]">"Insert Image into Chapter"</h3>
                        <button type="button" on:click=move |_| image_modal_open.set(false)>"Close ×"</button>
                    </div>
                    <div class="grid gap-2 md:grid-cols-3">
                        <input type="text" placeholder="Caption / Alt text" class="rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5" prop:value=move || new_image_title.get() on:input=move |ev| new_image_title.set(event_target_value(&ev))/>
                        <input type="text" placeholder="Image URL or Data URL" class="rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5" prop:value=move || new_image_url.get() on:input=move |ev| new_image_url.set(event_target_value(&ev))/>
                        <button
                            type="button"
                            class="rounded bg-[#5A5A40] text-white px-3 py-1.5"
                            on:click=move |_| {
                                let url = new_image_url.get().trim().to_string();
                                if url.is_empty() { return; }
                                let title = new_image_title.get().trim().to_string();
                                let label = if title.is_empty() { "Illustration".into() } else { title };
                                let nidx = state.active_novel_idx.get();
                                let mut created = None;
                                state.novels.update(|list| {
                                    if let Some(nv) = list.get_mut(nidx) {
                                        let asset = GalleryAsset {
                                            id: format!("img-{}", nv.gallery.len() + 1),
                                            title: label,
                                            data_url: url,
                                            width: 1024,
                                            height: 768,
                                            size_kb: 120,
                                        };
                                        nv.gallery.push(asset.clone());
                                        created = Some(asset);
                                    }
                                });
                                if let Some(a) = created { insert_asset_into_chapter(a); }
                                new_image_title.set(String::new());
                                new_image_url.set(String::new());
                            }
                        >"Save & Insert"</button>
                    </div>
                    <div class="grid gap-3 sm:grid-cols-2 md:grid-cols-3">
                        {move || state.novels.get().get(state.active_novel_idx.get()).map(|n| n.gallery.clone()).unwrap_or_default().into_iter().map(|asset| {
                            let a_ins = asset.clone();
                            view! {
                                <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-2 space-y-2">
                                    <img src=asset.data_url.clone() alt=asset.title.clone() class="w-full h-24 object-cover rounded"/>
                                    <div class="font-medium truncate">{asset.title.clone()}</div>
                                    <button type="button" class="w-full px-2 py-1 rounded bg-[#5A5A40] text-white" on:click=move |_| insert_asset_into_chapter(a_ins.clone())>"Insert at Cursor"</button>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </Show>
            }.into_any()
        }}
    }
}
