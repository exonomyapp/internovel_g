//! Pure Rust GFM Markdown-to-HTML renderer for Leptos CSR/SSR docs & prose preview.

use std::collections::HashMap;

use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown_html(markdown: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(markdown, opts);
    let mut html_output = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn render_markdown_with_assets(markdown: &str, assets: &HashMap<String, String>) -> String {
    let mut html_output = render_markdown_html(markdown);
    for (id, data_url) in assets {
        let needle = format!("src=\"internovel-img://{id}\"");
        if html_output.contains(&needle) {
            let escaped = data_url.replace('"', "&quot;");
            let replacement = format!("src=\"{escaped}\" class=\"rounded-xl max-h-96 my-2\"");
            html_output = html_output.replace(&needle, &replacement);
        }
    }
    html_output
}
