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

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header(class = "relative flex flex-row-reverse justify-center items-center px-16 py-6 text-emerald-500 text-shadow-lg shadow-emerald-400/75 font-extrabold p-4 backdrop-blur-lg text-center") {
            p(class = "text-2xl sm:text-4xl w-full absolute underline") { (title.to_string()) }
            nav(class = "text-[1.3rem] ml-auto mr-[20%]") {
                ul(class = "flex") {
                    li(class = "mx-12") {
                        a { (tl!(cx, "layout.links.blog", "Blog")) }
                    }
                }
            }
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
