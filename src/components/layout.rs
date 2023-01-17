use perseus::prelude::*;
use sycamore::prelude::*;

static COPYRIGHT_YEARS: &str = "2023";

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps { title, children }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header(class = "text-emerald-500 text-shadow-lg shadow-emerald-400/75 font-extrabold p-4 bg-white text-center underline") {
            p(class = "text-4xl") { (title.to_string()) }
        }
        main(style = "p-4") {
            (children)
        }
        // TODO Fix rogue diacritic here
        footer(class = "text-black bg-zinc-100 flex justify-center text-center p-4") {
            p(dangerously_set_inner_html = &t!(cx, "footer.text", { "years" = COPYRIGHT_YEARS })) {}
        }
    }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    /// The title of the page, which will be displayed in the header.
    pub title: String,
    /// The content to put inside the layout.
    pub children: Children<'a, G>,
}
