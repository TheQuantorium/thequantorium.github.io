use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

/// An access token for a FaunaDB instance that only has the permission to add new expressions
/// of interest.
static ACCESS_TOKEN: &str = "fnAE6i4BGcACWa5Uv95gQkIhlcFbe7B7U9Mi4yAy";

lazy_static::lazy_static! {
    /// Displays an interface for users to add their email and interests to an email list so
    /// they can be notified of developments with the network.
    pub static ref INTEREST_FORM: Capsule<PerseusNodeType, ()> = Capsule::build(
        Template::build("interest_form")
            .build_state_fn(get_build_state)
    )
        // No fallback because this is a one-page site
        .empty_fallback()
        .view_with_state(interest_form_widget)
        .build();
}

#[auto_scope]
fn interest_form_widget<G: Html>(cx: Scope, state: &InterestFormStateRx, _: ()) -> View<G> {
    // We always start in the unsubmitted state
    let form_state = create_signal(cx, FormState::Unsubmitted);

    view! { cx,
        div(class = "text-left my-12 flex flex-col items-center text-black dark:text-white") {
            form(class = "max-w-prose p-8 shadow-md rounded-xl") {
                h3(class = "text-4xl text-emerald-600 dark:text-emerald-400 text-shadow shadow-emerald-400/75 mb-2") { (t!(cx, "interest.heading")) }
                input(
                    class = "p-2 w-full border-2 border-emerald-600 rounded-md focus:border-emerald-400 focus:ring-0 focus:ring-offset-0 bg-transparent",
                    placeholder = t!(cx, "interest.email"),
                    bind:value = state.email,
                ) {}
                p(class = "italic text-zinc-600 dark:text-zinc-400 max-w-md") { (t!(cx, "interest.note")) }
                p { (t!(cx, "interest.interests-prelude")) }
                input(class = "accent-emerald-600", type = "checkbox", name = "general", disabled = true, checked = true) {}
                label(class = "ml-2", for = "general") { (t!(cx, "interest.general")) }
                br {}
                input(class = "accent-emerald-600", bind:checked = state.running_node, type = "checkbox", name = "running_node") {}
                label(class = "ml-2", for = "running_node") { (t!(cx, "interest.running_node")) }
                br {}
                input(class = "accent-emerald-600", bind:checked = state.app_dev, type = "checkbox", name = "app_dev") {}
                label(class = "ml-2", for = "app_dev") { (t!(cx, "interest.app_dev")) }
                br {}
                input(class = "accent-emerald-600", bind:checked = state.dev, type = "checkbox", name = "dev") {}
                label(class = "ml-2", for = "dev") { (t!(cx, "interest.dev")) }
                br {}
                input(class = "accent-emerald-600", bind:checked = state.donation, type = "checkbox", name = "donation") {}
                label(class = "ml-2", for = "donation") { (t!(cx, "interest.donation")) }
                br {}
                input(class = "accent-emerald-600", bind:checked = state.finance, type = "checkbox", name = "finance") {}
                label(class = "ml-2", for = "finance") { (t!(cx, "interest.finance")) }
                br {}
                // Any errors will be provided through the form state as strings (since they shoudl be human-readable first)
                p(class = "text-red-400") { (if let FormState::Error(err) = &*form_state.get() {
                        err.to_string()
                    } else {
                        String::new()
                }) }
                div(class = "w-full flex flex-col items-center") {
                    button(
                        type = "button",
                        class = "mt-2 px-6 p-3 text-white bg-emerald-500 shadow-lg shadow-emerald-500/50 rounded-lg font-semibold md:text-lg inline-flex items-center transition ease-in-out hover:scale-105",
                        on:click = move |_| {
                            #[cfg(client)]
                            {
                                use perseus::state::MakeUnrx;
                                let form_data = state.clone().make_unrx();
                                spawn_local_scoped(cx, async move {
                                    submit_form(cx, form_data, &form_state).await;
                                });
                            }
                        }
                    ) {
                        // When we're loading, show the user what's going on with a loader
                        (if let FormState::Loading = *form_state.get() {
                            view! { cx,
                                // Adapted for Sycamore from the TailwincCSS example at https://tailwindcss.com/docs/animation
                                svg(class = "animate-spin -ml-1 mr-3 h-5 w-5 text-white", xmlns = "http://www.w3.org/2000/svg", fill = "none", viewBox = "0 0 24 24") {
                                    circle(class = "opacity-25", cx = "12", cy = "12", r = "10", stroke = "currentColor", stroke-width = "4") {}
                                    path(class = "opacity-75", fill = "currentColor", d = "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z") {}
                                }
                            }
                        } else {
                            View::empty()
                        })
                        (t!(cx, "interest.submit"))
                    }
                    // All successes produce the same message
                    p(class = "text-emerald-500 mt-5 max-w-sm") { (if let FormState::Success = *form_state.get() {
                        t!(cx, "interest.form.success")
                    } else {
                        String::new()
                    }) }
                }
            }
        }
    }
}

/// Submits the interest form in the browser, given the form details (unreactively)
/// and an error signal to register errors in for the user.
#[cfg(client)]
async fn submit_form<'a>(
    cx: Scope<'a>,
    state: InterestFormState,
    form_state: &'a Signal<FormState>,
) {
    use gloo_net::http::Request;
    use regex::Regex;
    use serde_json::Value;
    use uuid::Uuid;

    // No matter what it previously was, we're starting a nedw attempt (this will lead
    // to a loader being displayed in the button)
    form_state.set(FormState::Loading);

    if state.email.is_empty() {
        form_state.set(FormState::Error(
            "Please provide an email address for us to send updates to!".to_string(),
        ));
        return;
    }

    // Now validate the email
    // See https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
    let email_regex = Regex::new(r#"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"#)
        .unwrap();
    if !email_regex.is_match(&state.email) {
        form_state.set(FormState::Error(t!(
            cx,
            "interest.form.error-invalid-email"
        )));
        return;
    }

    // Generate a random token for unsubscription (safe to do here, since the request
    // is encrypted, and the submitting user should be able to unsubscribe anyway!)
    let token = Uuid::new_v4().to_string();

    let gql_query = format!(
        r#"mutation {{
    createExpression(data: {{
        email: "{email}",
        token: "{token}",
        interests: {{
            runningNode: {running_node},
            appDev: {app_dev},
            dev: {dev},
            finance: {finance},
            donation: {donation},
        }}
    }}) {{ _id }}
}}"#,
        email = state.email,
        token = token,
        running_node = state.running_node,
        app_dev = state.app_dev,
        dev = state.dev,
        finance = state.finance,
        donation = state.donation,
    );

    let res = Request::post("https://graphql.fauna.com/graphql")
        .header("Authorization", &format!("Bearer {}", ACCESS_TOKEN))
        .json(&serde_json::json!({ "query": gql_query }))
        // We know the serialization of this will succeed (the query itself is already stringified)
        .unwrap()
        .send()
        .await;
    match res {
        Ok(res) if res.ok() => {
            match res.json::<Value>().await {
                Ok(val) => {
                    let id = val
                        .get("data")
                        .map(|data| data.get("createExpression").map(|data| data.get("_id")))
                        .flatten()
                        .flatten();
                    if id.is_some() {
                        // If this has worked, then we don't need to take any further action
                        // (because we don't bother to validate the new email, if they've given someone
                        // else's email address, they can always unsubscribe)
                        form_state.set(FormState::Success);
                    } else {
                        form_state.set(FormState::Error(t!(cx, "interest.form.error-parsing")))
                    }
                }
                Err(_) => form_state.set(FormState::Error(t!(cx, "interest.form.error-parsing"))),
            };
        }
        // Ok, but HTTP error returned in the status code
        //
        // This might mean we've eclipsed our allowances with Fauna
        Ok(res) => form_state.set(FormState::Error(t!(cx, "interest.form.error-http", {
            "http_code" = &res.status().to_string()
        }))),
        Err(_) => form_state.set(FormState::Error(t!(cx, "interest.form.error-request"))),
    }
}

#[derive(Serialize, Deserialize, Clone, Default, ReactiveState)]
#[rx(alias = "InterestFormStateRx")]
struct InterestFormState {
    email: String,
    // User interests
    running_node: bool,
    app_dev: bool,
    dev: bool,
    donation: bool,
    finance: bool,
}

/// A representation of the current state of the form with respect to its submission. This is not included in
/// the Perseus state because of the risk of caching a loading state before the logic that would cause a success
/// executes. If a use were to change the page then, they would return to a still-loading form later.
enum FormState {
    Success,
    Error(String),
    Loading,
    Unsubmitted,
}

#[engine_only_fn]
async fn get_build_state(_: StateGeneratorInfo<()>) -> InterestFormState {
    InterestFormState::default()
}
