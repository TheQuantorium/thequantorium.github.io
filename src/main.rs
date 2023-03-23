mod components;
mod error_views;
mod post_utils;
mod svg;
mod templates;

static COPYRIGHT_YEARS: &str = "2023";

use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::posts::get_template())
        .template(crate::templates::post::get_template())
        .capsule_ref(&*crate::components::INTEREST_FORM)
        .locales_and_translations_manager("en-US", &[])
        .index_view(|cx| {
            view! { cx,
                html {
                    head {
                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css") {}
                        link(rel = "stylesheet", href = ".perseus/static/layout.css") {}
                        meta(name = "viewport", content = "width=device-width, initial-scale=1") {}
                    }
                    body(class = "bg-white dark:bg-zinc-900") {
                        PerseusRoot {}
                        noscript(class = "fixed right-0 bottom-0 m-4 p-4 rounded-lg max-w-md bg-red-200 text-red-900") {
                            p { "This website will work much better with Javascript enabled! We promise to never track you." }
                        }
                    }
                }
            }
        })
        .error_views(crate::error_views::get_error_views())
}
