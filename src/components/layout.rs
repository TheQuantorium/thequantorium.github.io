use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        title,
        footer,
        children,
        i18ned,
    }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    macro_rules! tl {
        ($cx:ident, $id:literal, $fallback:literal) => {
            if i18ned {
                t!($cx, $id)
            } else {
                $fallback.to_string()
            }
        };
    }
    macro_rules! linkl {
        ($cx:ident, $link:literal, $fallback:literal) => {
            if i18ned {
                link!($cx, $link)
            } else {
                $fallback.to_string()
            }
        };
    }

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header(class = "relative flex flex-row justify-evenly items-center px-4 xs:px-8 sm:px-16 py-6 text-emerald-500 font-extrabold text-center") {
            span(class = "w-8 h-8 bg-black") {
                // TODO Icon
            }
            a(href = linkl!(cx, "", "en-US"), class = "text-2xl sm:text-4xl underline mx-2 text-shadow-lg hover:text-shadow-xl shadow-emerald-400/75 hover:shadow-emerald-500/75 transition-[text-shadow] transition-colors") { (title.to_string()) }
            nav(class = "text-lg sm:text-[1.3rem]") {
                ul(class = "flex") {
                    li {
                        a(class = "text-shadow-lg hover:text-shadow-xl shadow-emerald-400/75 hover:shadow-emerald-500/75 transition-[text-shadow] transition-colors", href = linkl!(cx, "posts", "en-US/posts")) { (tl!(cx, "layout.links.blog", "Blog")) }
                    }
                }
            }
            // TODO Mobile navbar
        }
        main(style = "p-4") {
            (children)
        }
        footer(class = "text-black dark:text-white bg-zinc-100 dark:bg-zinc-800 flex justify-center text-center p-4") {
            p(dangerously_set_inner_html = &footer) {}
        }
    }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    /// The title of the page, which will be displayed in the header.
    pub title: String,
    /// The text of the footer.
    pub footer: String,
    /// The content to put inside the layout.
    pub children: Children<'a, G>,
    /// Whether or not to use i18n (since this layout is also used for error views).
    pub i18ned: bool,
}
