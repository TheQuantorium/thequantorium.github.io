use crate::{components::Layout, COPYRIGHT_YEARS};
use fmterr::fmt_err;
use perseus::{error_views::ErrorPosition, errors::ClientError, prelude::*};
use sycamore::prelude::*;

// This site will be exported statically, so we only have control over 404 pages
// for broken links in the site itself
pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, pos| {
        match err {
            // Special case for 404 due to its frequency
            ClientError::ServerError { status, .. } if status == 404 => (
                view! { cx,
                    title { "Page not found" }
                },
                not_found_page(cx),
            ),
            ClientError::Panic(panic_msg) => (
                // Panics are popups
                View::empty(),
                view! { cx,
                        div(
                            class = "absolute bottom-0 right-0 bg-red-400 text-white m-4 rounded-lg max-w-[30rem]"
                        ) {
                            h2(
                                class = "text-2xl font-bold w-full border-b border-white my-4"
                            ) {
                                span(class = "pl-4") { "Critical error!" }
                            }
                            div(
                                class = "p-4 pt-0 mt-4"
                            ) {
                                p { "This website has panicked! Details are below if you'd like to report this to us (since this really shouldn't happen...)." }
                                pre(
                                    class = "bg-amber-500 p-4 mt-4 rounded-lg whitespace-pre-wrap",
                                    // TODO Tailwind doesn't support this?
                                    style = "word-wrap: break-word;"
                                ) {
                                    (panic_msg)
                                }
                            }
                        }
                },
            ),
            err => {
                let err_msg = fmt_err(&err);

                // This will be placed in either a popup or across the page
                let inner_view = view! { cx,
                    div(
                        class = "bg-red-400 text-white m-4 rounded-lg max-w-[30rem]"
                    ) {
                        h2(
                            class = "text-2xl font-bold w-full pb-4 border-b border-white my-4"
                        ) {
                            span(class = "pl-4") { "Error!" }
                        }
                        div(
                            class = "p-4 pt-0 mt-4"
                        ) {
                            p { "This website has encountered an internal error, sorry! Details are below if you'd like to report this to us." }
                            pre(
                                class = "bg-amber-500 p-4 mt-4 rounded-lg whitespace-pre-wrap",
                                style = "word-break: break-word;"
                            ) {
                                (err_msg)
                            }
                        }
                    }
                };

                (
                    view! { cx,
                            title { "Error" }
                    },
                    match pos {
                        ErrorPosition::Page => view! { cx,
                            Layout(title = "The Quantorium".to_string(), footer = format!("&copy; {} The Quantorium", COPYRIGHT_YEARS)) {
                                div(
                                    class = "flex flex-col justify-center items-center h-[95vh] w-full"
                                ) {
                                    (inner_view)
                                }
                            }
                        },
                        ErrorPosition::Popup => view! { cx,
                            div(
                                class = "absolute bottom-0 right-0 flex justify-center items-center"
                            ) {
                                (inner_view)
                            }
                        },
                        ErrorPosition::Widget => view! { cx,
                            div(
                                class = "flex flex-col"
                            ) {
                                (inner_view)
                            }
                        },
                    },
                )
            }
        }
    })
}

fn not_found_page<G: Html>(cx: Scope) -> View<G> {
    #[cfg(target_arch = "wasm32")]
    {
        // Attempt to redirect the user to Framesurge if they came for the Perseus website
        let location = web_sys::window().unwrap().location();
        let pathname = location.pathname();
        match pathname {
            Ok(pathname) if pathname.starts_with("/perseus") => {
                let redirect = pathname.replace("/perseus", "https://framesurge.sh/perseus");
                location.set_href(&redirect);
            }
            // Anything else is either a weird JS error or a genuine mistake from the user
            _ => (),
        };
    }

    view! { cx,
        Layout(title = "The Quantorium".to_string(), footer = format!("&copy; {} The Quantorium", COPYRIGHT_YEARS)) {
            div(class = "flex flex-col justify-center items-center h-screen") {
                main(class = "flex flex-col border-2 border-neutral-800 rounded-lg max-w-xl m-4") {
                    h3(class = "text-2xl font-bold w-full pb-4 border-b-2 border-neutral-800 my-4") {
                        span(class = "font-mono pl-4") { "404: Page not found!" }
                    }
                    div(class = "p-4 pt-0 my-4") {
                        span {
                            "That page doesn't seem to exist. If you came here another website, or a search engine, this page probably existed once, but has since been moved. Perhaps you'd like to go "
                            // Not ideal, but it works for now
                            a(href = "en-US", class = "underline text-blue-400 hover:text-blue-500 transition-colors duration-150") { "home" }
                            "?"
                        }
                    }
                }
            }
        }
    }
}
