use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_navigate,
    path,
};

use crate::pages::{
    auth::AuthPage,
    community::{CafePage, CastingPage, ProfilePage, SearchPage},
    docs::{DocViewerPage, DOC_FILES, USER_FILES},
    studio::StudioPage,
};
use crate::{AppState, EditorView, NovelWorkspace, StudioTab, ThemeMode, VertebraItem};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Local-first collaborative writing platform for authors, world-builders, and storytellers."/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <Stylesheet id="leptos" href="/pkg/internovel_server.css"/>
                <MetaTags/>
            </head>
            <body class="bg-[#f5f5f0] text-[#2c2c22] dark:bg-[#141412] dark:text-[#e6e6de] antialiased">
                <App/>
            </body>
        </html>
    }
}

/// Reusable non-recursive documentation dropdown component (per docs/agent/agent.md).
#[component]
fn DocMenuDropdown(
    label: &'static str,
    prefix: &'static str,
    items: &'static [(&'static str, &'static str, &'static str)],
) -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div class="relative">
            <button
                type="button"
                class="px-2.5 py-1.5 rounded-lg text-xs font-medium text-[#5A5A40] dark:text-[#d0d0c8] hover:bg-[#e5e5e0] dark:hover:bg-[#2d2d2a]"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                {label}
            </button>
            <Show when=move || open.get()>
                <div class="absolute left-0 mt-1 w-60 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-lg p-1.5 z-50">
                    <For
                        each=move || items.iter().copied()
                        key=|item| item.0
                        children=move |(slug, title, _)| {
                            let href = format!("{prefix}/{slug}");
                            view! {
                                <A
                                    href=href
                                    attr:class="block px-2.5 py-1.5 rounded-lg text-xs hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]"
                                    on:click=move |_| open.set(false)
                                >
                                    {title}
                                </A>
                            }
                        }
                    />
                    <a
                        href="/internovel-docs.zip"
                        download="internovel-docs.zip"
                        class="mt-1 block border-t border-[#e5e5e0] dark:border-[#2d2d2a] px-2.5 py-1.5 text-xs font-medium text-[#5A5A40] dark:text-[#d0d0c8] hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] rounded-b-lg"
                    >
                        "Download All Docs (.zip)"
                    </a>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let _state = AppState::provide();

    view! {
        <Title text="InterNovel"/>
        <Router>
            <AppShell/>
        </Router>
    }
}

#[component]
fn AppShell() -> impl IntoView {
    let state = expect_context::<AppState>();
    let navigate = use_navigate();
    let settings_open = RwSignal::new(false);
    let account_open = RwSignal::new(false);
    let new_novel_open = RwSignal::new(false);
    let new_novel_title = RwSignal::new(String::new());
    let new_novel_synopsis = RwSignal::new(String::new());
    let mobile_menu_open = RwSignal::new(false);

    let create_novel = move |_| {
        let title = new_novel_title.get().trim().to_string();
        if title.is_empty() {
            return;
        }
        state.snapshot_current_novel(None);
        let synopsis_raw = new_novel_synopsis.get().trim().to_string();
        let synopsis = if synopsis_raw.is_empty() {
            "New collaborative novel workspace.".to_string()
        } else {
            synopsis_raw
        };
        let n_idx = state.novels.get().len();
        let id = format!("novel-{}", n_idx + 1);
        let v_id = format!("v-init-{}", n_idx + 1);
        state.novels.update(|list| {
            list.push(NovelWorkspace {
                id,
                title: title.clone(),
                synopsis,
                collab_call: String::new(),
                seeking_collaborators: false,
                namespace_hex: "new..willow25".into(),
                forking_allowed: true,
                forking_locked: false,
                favorited: true,
                remixed_from_title: None,
                remixed_baseline_text: None,
                last_vertebra_id: v_id.clone(),
                last_studio_tab: StudioTab::Editor,
                last_editor_view: EditorView::Split,
                last_route: "/".into(),
                pitches: Vec::new(),
                gallery: Vec::new(),
                vertebrae: vec![VertebraItem {
                    id: v_id.clone(),
                    label: "Idea · Act I".into(),
                    kind: "Idea".into(),
                    title: "Chapter 1".into(),
                    storyline_date: "2026-01-01".into(),
                    last_edited_ts: 300,
                    child_count: 1,
                    tags: vec!["Draft".into()],
                    content: format!("# {title}\n\nStart writing..."),
                    plot_blocks: vec!["Opening premise".into()],
                }],
            });
            state.active_novel_idx.set(list.len() - 1);
        });
        state.selected_vertebra_id.set(v_id);
        state.studio_tab.set(StudioTab::Editor);
        state.editor_view.set(EditorView::Split);
        new_novel_title.set(String::new());
        new_novel_synopsis.set(String::new());
        new_novel_open.set(false);
    };

    view! {
        <div class=move || {
            match state.theme.get() {
                ThemeMode::Dark => "dark min-h-screen flex flex-col",
                _ => "min-h-screen flex flex-col",
            }
        }>
            <header class="sticky top-0 z-40 border-b border-[#e5e5e0] dark:border-[#2d2d2a] bg-white/95 dark:bg-[#1a1a18]/95 backdrop-blur px-4 py-2.5">
                <div class="max-w-7xl mx-auto flex flex-wrap items-center justify-between gap-2">
                    <div class="flex items-center gap-2">
                        <A href="/" attr:class="font-serif text-lg font-bold text-[#5A5A40] dark:text-[#e6e6de]">
                            "InterNovel"
                        </A>
                            <button
                                type="button"
                                class="md:hidden px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-xs"
                                on:click=move |_| mobile_menu_open.update(|v| *v = !*v)
                            >
                                "Menu"
                            </button>
                            <nav class="hidden md:flex items-center gap-1 text-xs">
                                <A href="/" attr:class="px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]">"Studio"</A>
                                <A href="/casting" attr:class="px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]">"Casting Studio"</A>
                                <A href="/cafe" attr:class="px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]">"Novel Café"</A>
                                <A href="/search" attr:class="px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]">"Search"</A>
                                <DocMenuDropdown label="Docs" prefix="/docs" items=DOC_FILES/>
                                <DocMenuDropdown label="User" prefix="/user" items=USER_FILES/>
                            </nav>
                        </div>

                        <div class="flex flex-wrap items-center gap-2">
                            // Novel Switcher + New Novel
                            <div class="flex items-center gap-1">
                                <select
                                    aria-label="Active Novel"
                                    class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1 text-xs"
                                    prop:value=move || state.active_novel_idx.get().to_string()
                                    on:change=move |ev| {
                                        if let Ok(idx) = event_target_value(&ev).parse::<usize>() {
                                            let dest = state.switch_active_novel(idx, None);
                                            if !dest.is_empty() {
                                                navigate(&dest, Default::default());
                                            }
                                        }
                                    }
                                >
                                    <For
                                        each=move || state.novels.get().into_iter().enumerate()
                                        key=|(_, n)| n.id.clone()
                                        children=move |(idx, novel)| {
                                            view! { <option value=idx.to_string()>{novel.title}</option> }
                                        }
                                    />
                                </select>
                                <button
                                    type="button"
                                    class="px-2 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-xs"
                                    on:click=move |_| new_novel_open.update(|v| *v = !*v)
                                >
                                    "+ Novel"
                                </button>
                            </div>

                            // Account Switcher Chip Dropdown
                            <div class="relative">
                                <button
                                    type="button"
                                    class="inline-flex items-center gap-2 rounded-full border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] pl-1.5 pr-3 py-1 text-xs font-medium"
                                    on:click=move |_| account_open.update(|v| *v = !*v)
                                >
                                    {move || match state.active_account.get() {
                                        Some(acc) => {
                                            let avatar = if !acc.avatar_url.is_empty() {
                                                view! {
                                                    <img
                                                        src=acc.avatar_url
                                                        alt=acc.display_name.clone()
                                                        class="w-5 h-5 rounded-full object-cover shrink-0 border border-[#e5e5e0] dark:border-[#3a3a35]"
                                                    />
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <span class="w-5 h-5 rounded-full bg-[#5A5A40] text-white text-[10px] inline-flex items-center justify-center shrink-0 font-bold">
                                                        {acc.avatar_initials}
                                                    </span>
                                                }.into_any()
                                            };
                                            view! {
                                                {avatar}
                                                <span>{acc.display_name}</span>
                                            }.into_any()
                                        }
                                        None => view! { <span class="px-1.5">"Unlock / Create Wallet"</span> }.into_any(),
                                    }}
                                </button>
                                <Show when=move || account_open.get()>
                                    <div class="absolute right-0 mt-1 w-64 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-lg p-2 z-50 text-xs space-y-1">
                                        <For
                                            each=move || state.accounts.get()
                                            key=|a| a.did_short.clone()
                                            children=move |acc| {
                                                let acc_clone = acc.clone();
                                                let avatar = if !acc.avatar_url.is_empty() {
                                                    view! {
                                                        <img
                                                            src=acc.avatar_url.clone()
                                                            alt=acc.display_name.clone()
                                                            class="w-6 h-6 rounded-full object-cover shrink-0 border border-[#e5e5e0] dark:border-[#3a3a35]"
                                                        />
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <span class="w-6 h-6 rounded-full bg-[#5A5A40] text-white text-[10px] inline-flex items-center justify-center shrink-0 font-bold">
                                                            {acc.avatar_initials.clone()}
                                                        </span>
                                                    }.into_any()
                                                };
                                                view! {
                                                    <button
                                                        type="button"
                                                        class="w-full text-left px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] flex items-center gap-2"
                                                        on:click=move |_| {
                                                            state.active_account.set(Some(acc_clone.clone()));
                                                            account_open.set(false);
                                                        }
                                                    >
                                                        {avatar}
                                                        <div class="min-w-0 flex-1">
                                                            <div class="font-medium truncate">{acc.display_name}</div>
                                                            <div class="text-[10px] opacity-70 truncate">{acc.did_short}</div>
                                                        </div>
                                                    </button>
                                                }
                                            }
                                        />
                                        <A
                                            href="/profile"
                                            attr:class="block px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]"
                                            on:click=move |_| account_open.set(false)
                                        >
                                            "Author Profile & Key Export"
                                        </A>
                                        <A
                                            href="/auth"
                                            attr:class="block px-2.5 py-1.5 rounded-lg font-medium text-[#5A5A40] dark:text-[#d0d0c8] hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]"
                                            on:click=move |_| account_open.set(false)
                                        >
                                            "Create / Switch Wallet →"
                                        </A>
                                    </div>
                                </Show>
                            </div>

                            // Settings Dropdown (Theme, 25%..300% Text Scale, Iroh Relay, Backup Schedule)
                            <div class="relative">
                                <button
                                    type="button"
                                    class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] px-2.5 py-1 text-xs hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]"
                                    on:click=move |_| settings_open.update(|v| *v = !*v)
                                >
                                    "Settings"
                                </button>
                                <Show when=move || settings_open.get()>
                                    <div class="absolute right-0 mt-1 w-72 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-lg p-3 z-50 text-xs space-y-3">
                                        <div>
                                            <div class="font-medium mb-1">"Theme"</div>
                                            <div class="grid grid-cols-3 gap-1">
                                                <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| state.theme.set(ThemeMode::Light)>"Light"</button>
                                                <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| state.theme.set(ThemeMode::Dark)>"Dark"</button>
                                                <button type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a]" on:click=move |_| state.theme.set(ThemeMode::System)>"System"</button>
                                            </div>
                                        </div>
                                        <div>
                                            <div class="flex justify-between font-medium mb-1">
                                                <span>"Text Scale"</span>
                                                <span>{move || format!("{}%", state.text_scale.get())}</span>
                                            </div>
                                            <input
                                                type="range"
                                                min="25"
                                                max="300"
                                                step="10"
                                                class="w-full"
                                                prop:value=move || state.text_scale.get().to_string()
                                                on:input=move |ev| {
                                                    if let Ok(v) = event_target_value(&ev).parse::<u16>() {
                                                        state.text_scale.set(v);
                                                    }
                                                }
                                            />
                                        </div>
                                        <div>
                                            <label class="block font-medium mb-1">"Always-On Relay URL (Optional)"</label>
                                            <input
                                                type="text"
                                                placeholder="https://relay.exosystems.net"
                                                class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1"
                                                prop:value=move || state.iroh_relay_url.get()
                                                on:input=move |ev| state.iroh_relay_url.set(event_target_value(&ev))
                                            />
                                        </div>
                                        <div>
                                            <label class="block font-medium mb-1">"Local Backup Schedule"</label>
                                            <select
                                                class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1"
                                                on:change=move |ev| state.backup_schedule.set(event_target_value(&ev))
                                            >
                                                <option value="Daily">"Daily"</option>
                                                <option value="Weekly" selected=true>"Weekly"</option>
                                                <option value="Manual">"Manual Only"</option>
                                            </select>
                                        </div>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    </div>

                    <Show when=move || mobile_menu_open.get()>
                        <nav class="md:hidden pt-2 mt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] flex flex-wrap gap-1.5 text-xs">
                            <A href="/" attr:class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522]">"Studio"</A>
                            <A href="/casting" attr:class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522]">"Casting Studio"</A>
                            <A href="/cafe" attr:class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522]">"Novel Café"</A>
                            <A href="/search" attr:class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522]">"Search"</A>
                            <A href="/auth" attr:class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522]">"Wallet Auth"</A>
                            <DocMenuDropdown label="Docs" prefix="/docs" items=DOC_FILES/>
                            <DocMenuDropdown label="User" prefix="/user" items=USER_FILES/>
                        </nav>
                    </Show>

                    <Show when=move || new_novel_open.get()>
                        <div class="max-w-7xl mx-auto mt-2 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] flex flex-wrap items-center gap-2 text-xs">
                            <input
                                type="text"
                                placeholder="New novel title..."
                                class="flex-1 min-w-[180px] rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1"
                                prop:value=move || new_novel_title.get()
                                on:input=move |ev| new_novel_title.set(event_target_value(&ev))
                            />
                            <input
                                type="text"
                                placeholder="Short synopsis (optional)..."
                                class="flex-1 min-w-[200px] rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1"
                                prop:value=move || new_novel_synopsis.get()
                                on:input=move |ev| new_novel_synopsis.set(event_target_value(&ev))
                            />
                            <button type="button" class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white" on:click=create_novel>"Create Novel"</button>
                        </div>
                    </Show>
                </header>

                <main class="flex-1 max-w-7xl w-full mx-auto p-4 md:p-6">
                    <Routes fallback=|| view! { <p class="text-sm">"Page not found."</p> }>
                        <Route path=path!("/") view=StudioPage/>
                        <Route path=path!("/auth") view=AuthPage/>
                        <Route path=path!("/profile") view=ProfilePage/>
                        <Route path=path!("/casting") view=CastingPage/>
                        <Route path=path!("/cafe") view=CafePage/>
                        <Route path=path!("/search") view=SearchPage/>
                        <Route path=path!("/docs/:slug") view=DocViewerPage/>
                        <Route path=path!("/user/:slug") view=DocViewerPage/>
                    </Routes>
                </main>
        </div>
    }
}
