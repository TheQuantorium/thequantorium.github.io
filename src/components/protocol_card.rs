use sycamore::prelude::*;

/// A card for a single one of the Quantorium's protocols.
#[component]
pub fn ProtocolCard<G: Html>(
    cx: Scope,
    ProtocolCardProps {
        name,
        description,
        link,
        emoji,
    }: ProtocolCardProps,
) -> View<G> {
    view! { cx,
        div(class = "p-8 shadow-md rounded-lg m-4 sm:max-w-xl") {
            a(
                // We apply all the emoji styling to the text as well to sync the hovers
                class = "text-2xl sm:text-3xl font-bold emoji-emerald transition-[text-shadow]",
                href = link
            ) {
                span(class = "mr-2", dangerously_set_inner_html = &emoji) {}
                span { (name) }
            }
            p(class = "sm:text-lg xl:text-xl text-zinc-600 dark:text-zinc-400 font-semibold max-w-prose mt-2") { (description) }
        }
    }
}

#[derive(Prop)]
pub struct ProtocolCardProps {
    name: String,
    description: String,
    link: String,
    emoji: String,
}
