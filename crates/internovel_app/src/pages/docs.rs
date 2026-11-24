use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::markdown::render_markdown_html;
use crate::AppState;

/// Compile-time embedded root `/docs/*.md` development specs (excludes `/docs/agent/`).
pub const DOC_FILES: &[(&str, &str, &str)] = &[
    ("willow-dev", "Willow Dev", include_str!("../../../../docs/willow-dev.md")),
    ("willow-gap", "Willow Gap", include_str!("../../../../docs/willow-gap.md")),
    ("identity-spec", "Identity Spec", include_str!("../../../../docs/identity-spec.md")),
    ("technical-overview", "Technical Overview", include_str!("../../../../docs/technical-overview.md")),
    ("deployment-packaging", "Deployment & Packaging", include_str!("../../../../docs/deployment-packaging.md")),
    ("functional-spec", "Functional Spec", include_str!("../../../../docs/functional-spec.md")),
    ("search-and-discovery", "Search & Discovery", include_str!("../../../../docs/search-and-discovery.md")),
    ("feature-roadmap", "Feature Roadmap", include_str!("../../../../docs/feature-roadmap.md")),
    ("vision-scope", "Vision Scope", include_str!("../../../../docs/vision-scope.md")),
];

/// Compile-time embedded `/docs/user/*.md` user guides.
pub const USER_FILES: &[(&str, &str, &str)] = &[
    ("intro", "Intro", include_str!("../../../../docs/user/intro.md")),
    ("search", "Search", include_str!("../../../../docs/user/search.md")),
];

#[component]
pub fn DocViewerPage() -> impl IntoView {
    let state = expect_context::<AppState>();
    let params = use_params_map();

    view! {
        <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 md:p-8 space-y-4">
            <div class="flex justify-end">
                <a
                    href="/internovel-docs.zip"
                    download="internovel-docs.zip"
                    class="inline-flex items-center gap-1.5 rounded-lg bg-[#f5f5f0] dark:bg-[#2d2d2a] px-3 py-1.5 text-xs font-medium text-[#5A5A40] dark:text-[#d0d0c8]"
                >
                    "Download All Docs (.zip)"
                </a>
            </div>
            {move || {
                let slug = params.read().get("slug").unwrap_or_default();
                let clean_slug = slug.trim_end_matches(".md");
                let found = DOC_FILES
                    .iter()
                    .chain(USER_FILES.iter())
                    .find(|(s, _, _)| *s == clean_slug);

                match found {
                    Some((_, _title, body)) => {
                        let rendered_html = render_markdown_html(body);
                        view! {
                            <article
                                class="prose prose-stone dark:prose-invert prose-micro max-w-none"
                                style=move || format!("font-size: {}%;", state.text_scale.get())
                                inner_html=rendered_html
                            />
                        }.into_any()
                    }
                    None => view! {
                        <p class="text-sm">"Document not found."</p>
                    }.into_any(),
                }
            }}
        </div>
    }
}
