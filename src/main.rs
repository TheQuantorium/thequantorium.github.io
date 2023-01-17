mod components;
mod svg;
mod templates;

use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .locales_and_translations_manager("en-US", &[])
        .index_view(|cx| {
            view! { cx,
                html {
                    head {
                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css") {}
                        link(rel = "stylesheet", href = ".perseus/static/layout.css") {}
                    }
                    body {
                        PerseusRoot {}
                    }
                }
            }
        })
}
