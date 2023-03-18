use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use sycamore::prelude::*;

use crate::{
    components::{Layout, ProtocolCard, Typewriter, INTEREST_FORM},
    svg::BOOK,
    COPYRIGHT_YEARS,
};

#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct IntroState {
    html_intro: String,
}

fn index_page<G: Html>(cx: Scope, IntroState { html_intro }: IntroState) -> View<G> {
    let typewriter_is_done = create_rc_signal(false);
    let tid = typewriter_is_done.clone();
    let tw_span_classes = create_memo(cx, move || {
        let done = tid.clone().get();
        if *done {
            // Once we're done, turn red
            "text-red-600 dark:text-red-500 text-shadow-lg shadow-red-400/75 transition-colors transition-[text-shadow] duration-300 underline"
        } else {
            // Same green as the rest of the heading until we're done
            "text-emerald-600 dark:text-emerald-500 text-shadow-lg shadow-emerald-400/75 transition-colors transition-[text-shadow] duration-300"
        }.to_string()
    });

    view! { cx,
        Layout(title = t!(cx, "the-quantorium"), footer = t!(cx, "footer.text", { "years" = COPYRIGHT_YEARS }), i18ned = true) {
            // Title card
            div(class = "flex flex-col justify-center items-center py-24 px-6 xs:px-12 sm:px-20 md:px-32 lg:px-48 text-center") {
                h1(class = "text-3xl sm:text-5xl md:text-6xl font-extrabold") {
                    span(
                        class = "text-emerald-600 dark:text-emerald-500 text-shadow-lg shadow-emerald-400/75"
                    ) { (t!(cx, "index.heading.preamble")) }
                    span(
                        class = tw_span_classes.get()
                    ) {
                        Typewriter(
                            strings = vec! [
                                t!(cx, "index.heading.tw.1"),
                                t!(cx, "index.heading.tw.2"),
                                t!(cx, "index.heading.tw.3"),
                                t!(cx, "index.heading.tw.4"),
                                t!(cx, "index.heading.tw.5"),
                                t!(cx, "index.heading.tw.6"),
                                t!(cx, "index.heading.tw.7"),
                            ],
                            phrase_delay = 1000,
                            char_delay = 75,
                            cyclical = false,
                            is_done = typewriter_is_done
                        )
                    }
                }
                p(
                    class = "text-lg sm:text-xl font-semibold text-emerald-600 dark:text-emerald-400 mt-8",
                    dangerously_set_inner_html = &t!(cx, "index.summary")
                ) {}
            }

            // Cards for each protocol
            div(class = "flex flex-col lg:flex-row justify-center items-center lg:items-start mx-4 sm:mx-10 mb-24") {
                ProtocolCard(
                    name = t!(cx, "index.proto.lykros.name"),
                    description = t!(cx, "index.proto.lykros.desc"),
                    emoji = "&#x1F512;".to_string(),
                    link = "https://github.com/TheQuantorium/lykros".to_string(),
                )
                ProtocolCard(
                    name = t!(cx, "index.proto.kolaris.name"),
                    description = t!(cx, "index.proto.kolaris.desc"),
                    emoji = "&#x1F5A7;".to_string(),
                    link = "https://github.com/TheQuantorium/kolaris".to_string(),
                )
                ProtocolCard(
                    name = t!(cx, "index.proto.miriar.name"),
                    description = t!(cx, "index.proto.miriar.desc"),
                    emoji = "&#x1F4C3;".to_string(),
                    link = "https://github.com/TheQuantorium/miriar".to_string(),
                )
            }

            // A form for users to express their interest in the Quantorium
            (INTEREST_FORM.widget(cx, "", ()))

            // The full introduction
            div(class = "flex flex-col items-center m-4 sm:mx-8 md:mx-16") {
                h2(class = "text-4xl text-emerald-600 dark:text-emerald-400 text-shadow-lg shadow-emerald-400/75 mb-4") { (t!(cx, "intro.heading")) }
                p(
                    class = "max-w-prose mx-2 text-black dark:text-white md:text-lg",
                    dangerously_set_inner_html = &html_intro
                ) {}
                (INTEREST_FORM.widget(cx, "", ()))
                // a(
                //     class = "my-2 p-4 text-white bg-emerald-500 shadow-lg shadow-emerald-500/50 rounded-lg font-semibold md:text-lg inline-flex items-center transition ease-in-out hover:-translate-y-1 hover:scale-105",
                //     href = "https://github.com/TheQuantorium/manifesto/tree/main/manifesto.pdf",
                //     target = "_blank"
                // ) {
                //     span(class = "fill-white mr-2", dangerously_set_inner_html = BOOK) {}
                //     " "
                //     (t!(cx, "intro.cta"))
                // }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { (t!(cx, "the-quantorium")) }
        link(rel = "stylesheet", href = ".perseus/static/index.css")
    }
}

#[engine_only_fn]
async fn get_build_state(_: StateGeneratorInfo<()>) -> Result<IntroState, BlamedError<io::Error>> {
    use pulldown_cmark::{html, Options, Parser};
    use tokio::fs;

    let md_content = fs::read_to_string("intro.md").await?;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&md_content, opts);
    let mut html_intro = String::new();
    html::push_html(&mut html_intro, parser);

    Ok(IntroState { html_intro })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .view_with_unreactive_state(index_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}
