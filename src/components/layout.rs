use perseus::prelude::*;
use sycamore::prelude::*;

static LOGO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path stroke="currentColor" fill="currentColor" stroke-width="0.5" d="M26.226,33.98a4.533,4.533,0,0,0,4.3,2.675,5.948,5.948,0,0,0,1.276-.149c.3-.084.508-.075.625.024a.43.43,0,0,1,0,.651,2.545,2.545,0,0,1-.25.149,11.015,11.015,0,0,1-4.276.875,7.238,7.238,0,0,1-1.4-.149,7.091,7.091,0,0,1-4.175-2.476,5.566,5.566,0,0,1-.875-1.35l-.2-.375-.425-.15a8.256,8.256,0,0,1-4.187-3.074,9.247,9.247,0,0,1-1.513-5.45,9.421,9.421,0,0,1,1.726-5.776,7.224,7.224,0,0,1,1.375-1.4A8.789,8.789,0,0,1,23.7,16.155q1.2,0,1.476.026a9.226,9.226,0,0,1,4.6,1.825,8.565,8.565,0,0,1,3.1,7.149,9.488,9.488,0,0,1-1.725,5.776,7.253,7.253,0,0,1-1.375,1.4,8.362,8.362,0,0,1-3.224,1.551ZM18.65,25.155q0-5.148,1.8-7.325a.57.57,0,0,0,.1-.125c.015-.033.015-.05,0-.05a2.458,2.458,0,0,0-.875.425q-3.7,2.226-3.7,7.1,0,4.924,3.7,7.075a3.957,3.957,0,0,0,.9.425c-.033-.05-.075-.109-.125-.175a7.253,7.253,0,0,1-1.4-3.151A18.389,18.389,0,0,1,18.65,25.155Zm6.1-8.075a6.628,6.628,0,0,0-.9-.05,3.277,3.277,0,0,0-2.4,1q-1.924,1.875-1.925,7.151a13.075,13.075,0,0,0,1.026,5.849,3.931,3.931,0,0,0,2.224,2.125,5.049,5.049,0,0,0,2.451,0q2.349-.825,3.024-4.774a18.544,18.544,0,0,0,.226-3.226,17.847,17.847,0,0,0-.375-3.95Q27.276,17.532,24.75,17.08Zm-1.9,17.075a2.433,2.433,0,0,1-.425-.075h-.075l.049.125a6.147,6.147,0,0,0,2.625,2.426,6.329,6.329,0,0,0,3,.7,2.025,2.025,0,0,0,.725-.05.985.985,0,0,0-.225-.1,5.374,5.374,0,0,1-2.375-1.726,5.253,5.253,0,0,1-.674-1.05l-.125-.274H25.2a9.194,9.194,0,0,1-1.274.074Q23.449,34.205,22.851,34.155Zm6.5-9q0,5.2-1.925,7.526a3.979,3.979,0,0,0,.9-.425Q32,30.1,32,25.155a7.975,7.975,0,0,0-2.149-5.9,7.191,7.191,0,0,0-2.1-1.476,2.911,2.911,0,0,0-.324-.125c.033.05.074.109.125.175a7.26,7.26,0,0,1,1.4,3.15A18.331,18.331,0,0,1,29.351,25.155Z"/></svg>"#;

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
            a(
                href = linkl!(cx, "", "en-US"),
                class = "w-16 h-16 fill-emerald-600 dark:fill-emerald-400",
                dangerously_set_inner_html = LOGO,
            ) {}
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
