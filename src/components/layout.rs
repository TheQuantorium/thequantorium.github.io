use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        title,
        footer,
        children,
    }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header(class = "text-emerald-500 text-shadow-lg shadow-emerald-400/75 font-extrabold p-4 backdrop-blur-lg text-center underline") {
            p(class = "text-2xl sm:text-4xl") { (title.to_string()) }
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
}
