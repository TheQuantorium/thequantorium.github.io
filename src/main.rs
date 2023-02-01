mod components;
mod error_views;
mod svg;
mod templates;

static COPYRIGHT_YEARS: &str = "2023";

use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
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
                    }
                }
            }
        })
        .error_views(crate::error_views::get_error_views())
}
