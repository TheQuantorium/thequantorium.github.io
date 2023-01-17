//! Typewriter effects for the landing page.

use sycamore::prelude::*;

/// A typewriter effect, which will first display the first element in the given
/// list, and that will then backspace that and replace it with the next one, etc.
///
/// This will only work for unicode strings!
#[component]
pub fn Typewriter<G: Html>(
    cx: Scope,
    TypewriterProps {
        strings,
        #[cfg(client)]
        phrase_delay,
        #[cfg(client)]
        char_delay,
        #[cfg(client)]
        cyclical,
        #[cfg(client)]
        is_done,
        ..
    }: TypewriterProps,
) -> View<G> {
    // This would be caught at build-time
    assert!(
        strings.len() >= 2,
        "must provide at least two strings to cycle through"
    );

    let text = create_rc_signal(strings[0].to_string());
    let text_ref = create_ref(cx, text.clone());

    #[cfg(client)]
    {
        use perseus::prelude::spawn_local_scoped;

        let mut strings: Box<dyn Iterator<Item = String>> = if cyclical {
            // This will now repeat forever with `.next()`
            Box::new(strings.into_iter().cycle())
        } else {
            Box::new(strings.into_iter())
        };

        // We're already displaying the first string, so advance the iterator
        let _ = strings.next();

        let typewriter_status = create_rc_signal(TypewriterStatus::Backward(
            strings.next().unwrap().chars().collect::<Vec<char>>(),
        ));
        let mut interval_handle: Option<i32> = None;

        spawn_local_scoped(cx, async move {
            use gloo_timers::callback::Interval;
            use std::time::Duration;

            // Before we even start, delay so that the user can see the current word
            gloo_timers::future::sleep(Duration::from_millis(phrase_delay.into())).await;

            // We immediately 'forget' the interval according to RAII, and then
            // we acquire a handle to it that can be used to cancel it as part
            // of scope cleanup.
            interval_handle = Some(
                Interval::new(char_delay, move || {
                    let mut curr_text = (*text.get()).to_string();
                    let status = typewriter_status.get();

                    match &*status {
                        TypewriterStatus::Forward(chars) => {
                            let mut chars = chars.to_vec();
                            let first_char = chars.remove(0);
                            curr_text.push(first_char);
                            // Remove this char from the current list
                            if !chars.is_empty() {
                                typewriter_status.set(TypewriterStatus::Forward(chars));
                            } else {
                                // We've reached the end of this word, start a delay (which
                                // should be divided by the length of this interval's timer to
                                // get the number of passes we should be waiting for).
                                typewriter_status
                                    .set(TypewriterStatus::Waiting(phrase_delay / char_delay));
                            }
                            text.set(curr_text);
                        }
                        TypewriterStatus::Backward(next_string_chars) => {
                            curr_text.pop();
                            // If we've finished backspacing the word, start the next one
                            if curr_text.is_empty() {
                                typewriter_status
                                    .set(TypewriterStatus::Forward(next_string_chars.to_vec()));
                            }

                            // If this is empty, CSS will make sure it keeps its height
                            text.set(curr_text);
                        }
                        TypewriterStatus::Waiting(iters_left) => {
                            // We're waiting for a certain number of iterations, so just decrement
                            // it before we go vto backspacing
                            let new_iters_left = iters_left - 1;
                            if new_iters_left == 0 {
                                // Cyclic iterators never return `None` from `.next()`
                                if let Some(next_string) = strings.next() {
                                    let next_chars = next_string.chars().collect::<Vec<char>>();
                                    typewriter_status.set(TypewriterStatus::Backward(next_chars));
                                } else {
                                    typewriter_status.set(TypewriterStatus::Done);
                                    is_done.set(true);
                                }
                            } else {
                                typewriter_status.set(TypewriterStatus::Waiting(new_iters_left));
                            }
                        }
                        // If we're done, just keep looping until the user leaves the page
                        TypewriterStatus::Done => (),
                    }
                })
                .forget(),
            );

            // We should never get here unless the interval fails and drops its `tx`
        });

        // Clear the interval by its handle when the scope is destroyed
        on_cleanup(cx, move || {
            if let Some(handle) = interval_handle {
                web_sys::window()
                    .unwrap()
                    .clear_interval_with_handle(handle);
            }
        });
    }

    view! { cx,
        // We use a `p` rather than a `span` to force a line break (used in this
        // case at least, maybe not so generally)
        p(class = "p-keep-height") { (text_ref.get()) }
    }
}

#[derive(Prop)]
pub struct TypewriterProps {
    /// The list of strings to re-type, in order.
    strings: Vec<String>,
    /// The delay in milliseconds to leave a typed phrase up before backspacing
    /// it and typing the next one.
    phrase_delay: u32,
    /// The delay in milliseconds between typing each character.
    char_delay: u32,
    /// Whether or not the typewriter should be cyclical (i.e. keep cycling
    /// through the options after they've finished).
    cyclical: bool,
    /// A signal to be set to `true` when the typewriter is done, if `cyclical`
    /// is `false`. This can allow extra styling once the typewriter is complete,
    /// or the like.
    is_done: RcSignal<bool>,
}

#[cfg(client)]
enum TypewriterStatus {
    /// The next action should be to type an extra character.
    Forward(Vec<char>),
    /// The next action should be to remove the last character. When we're done, go
    /// to `Forward` with the given list of characters.
    Backward(Vec<char>),
    /// We've typed a new phrase fully, and now we're waiting so that the user
    /// can read it. This will wait for a certain number of iterations of 100ms,
    /// and then the interval will go to `TypewriterStatus::Backward`.
    ///
    /// Attached is the number of iterations still remaining, which will be decremented
    /// each time.
    Waiting(u32),
    /// The typewriter has finished typing all the phrases (non-cyclical option
    /// selected).
    Done,
}
