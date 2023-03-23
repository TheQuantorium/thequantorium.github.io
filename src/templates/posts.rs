use crate::post_utils::{format_date, SlimPost};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::{
    components::{Layout, INTEREST_FORM},
    COPYRIGHT_YEARS,
};

#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct PostsState {
    posts: Vec<SlimPost>,
}

fn posts_page<G: Html>(cx: Scope, PostsState { posts }: PostsState) -> View<G> {
    let locale = Reactor::<G>::from_cx(cx).get_translator().get_locale();

    let posts_view = View::new_fragment(
        posts.into_iter()
            .map(|post| {
                let localized_date = format_date(post.date.0, post.date.1, post.date.2);
                let localized_link = format!("{}/post/{}", locale, post.slug);
                view! {
                    cx,
                    li {
                        a(href = localized_link, class = "flex flex-col p-8 rounded-lg max-w-xl shadow-lg hover:shadow-xl transition-shadow") {
                            span(class = "text-xl font-bold text-emerald-500 text-shadow-sm shadow-emerald-400/75") { (post.title) }
                            span(class = "italic text-neutral-700 dark:text-neutral-400") { (t!(cx, "post.date", { "date" = &localized_date })) }
                            span(class = "mt-2 dark:text-white") { (post.description) }
                        }
                    }
                }
            })
            .collect()
    );
    view! { cx,
        Layout(title = t!(cx, "the-quantorium"), footer = t!(cx, "footer.text", { "years" = COPYRIGHT_YEARS }), i18ned = true) {
            div(class = "flex flex-col items-center mx-4 mt-4 md:mt-10 lg:mt-12") {
                div {
                    h1(class = "ml-4 text-2xl font-bold text-emerald-500 text-shadow-sm shadow-emerald-400/75") { (t!(cx, "posts.intro")) }
                    ul(class = "mt-4") {
                        (posts_view)
                    }
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { (t!(cx, "posts.title")) }
    }
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { locale, .. }: StateGeneratorInfo<()>,
) -> Result<PostsState, BlamedError<anyhow::Error>> {
    use crate::post_utils::parse_slim_post;
    use std::fs;

    let mut posts = Vec::new();

    // We want to read post info correctly based on the locale
    let localized_dir = format!("blog/{}", locale);
    for entry in fs::read_dir(localized_dir).map_err(anyhow::Error::from)? {
        let entry = entry.map_err(anyhow::Error::from)?;
        let file_path = entry.path();

        // Skip the README.md file
        if file_path.file_name() == Some("README.md".as_ref()) {
            continue;
        }

        // Only process Markdown files
        if file_path.extension() == Some("md".as_ref()) {
            let slim_post = parse_slim_post(&file_path)?;
            posts.push(slim_post);
        }
    }

    Ok(PostsState { posts })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("posts")
        .view_with_unreactive_state(posts_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}
