use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use sycamore::prelude::*;

use crate::{
    components::{Layout, INTEREST_FORM},
    post_utils::{format_date, Post},
    COPYRIGHT_YEARS,
};

#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct PostState {
    post: Post,
}

fn post_page<G: Html>(cx: Scope, PostState { post }: PostState) -> View<G> {
    // This calls out to the browser's
    let localized_date = format_date(post.date.0, post.date.1, post.date.2);

    view! { cx,
        Layout(title = t!(cx, "the-quantorium"), footer = t!(cx, "footer.text", { "years" = COPYRIGHT_YEARS }), i18ned = true) {
            div(class = "flex justify-center mx-2 mt-4 md:mt-10 lg:mt-12") {
                div(class = "max-w-prose mx-4 dark:text-white") {
                    h1(class = "text-2xl md:text-3xl lg:text-4xl font-bold") { (post.title) }
                    span(class = "italic text-neutral-700 dark:text-neutral-400") { (t!(cx, "post.date", { "date" = &localized_date })) }
                    div(class = "mt-6 markdown", dangerously_set_inner_html = &post.contents) {}
                }
            }

            (INTEREST_FORM.widget(cx, "", ()))
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, PostState { post }: PostState) -> View<SsrNode> {
    let page_title = format!("{} | {}", post.title, t!(cx, "the-quantorium"));

    // TODO Add support for syntax highlighting when we need it
    view! { cx,
        title { (page_title) }
        link(rel = "stylesheet", href = ".perseus/static/markdown.css") {}
        link(rel = "stylesheet", href = ".perseus/static/glow.css") {}
    }
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, locale, .. }: StateGeneratorInfo<()>,
) -> Result<PostState, BlamedError<anyhow::Error>> {
    use crate::post_utils::parse_full_post;
    use std::path::Path;

    let file_path = format!("blog/{}/{}.md", locale, path);
    let file_path = Path::new(&file_path);
    let post = parse_full_post(file_path)?;

    Ok(PostState { post })
}
#[engine_only_fn]
async fn get_build_paths() -> Result<BuildPaths, io::Error> {
    use std::fs;

    let mut post_slugs = Vec::new();

    // We'll go off the English posts to glean the slugs of the posts (which should stay the same)
    for entry in fs::read_dir("blog/en-US")? {
        let entry = entry?;
        let file_path = entry.path();

        // Only process Markdown files
        if file_path.extension() == Some("md".as_ref()) {
            let slug = file_path.file_name().unwrap().to_string_lossy().to_string();
            // Obviously guaranteed to be here in this conditional
            let slug = slug.strip_suffix(".md").unwrap();
            post_slugs.push(slug.to_string());
        }
    }

    Ok(BuildPaths {
        paths: post_slugs,
        extra: ().into(),
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("post")
        .view_with_unreactive_state(post_page)
        .head_with_state(head)
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
        .build()
}
