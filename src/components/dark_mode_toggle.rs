use crate::functions::dark_mode::ToggleDarkMode;
use leptos::*;
use leptos_router::{ActionForm, ActionFormProps};

#[derive(Debug, Clone)]
pub struct ColorScheme(pub RwSignal<bool>);

#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark(_cx: Scope) -> bool {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains("darkmode=true")
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark(cx: Scope) -> bool {
    use axum_extra::extract::cookie::CookieJar;
    use_context::<leptos_axum::RequestParts>(cx)
        .and_then(|req| {
            let cookies = CookieJar::from_headers(&req.headers);
            cookies.get("darkmode").and_then(|v| match v.value() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            })
        })
        .unwrap_or(false)
}

#[component]
pub fn DarkModeToggle(cx: Scope) -> impl IntoView {
    let initial = initial_prefers_dark(cx);

    let toggle_dark_mode_action = create_server_action::<ToggleDarkMode>(cx);
    // input is `Some(value)` when pending, and `None` if not pending
    let input = toggle_dark_mode_action.input();
    // value contains most recently-returned value
    let value = toggle_dark_mode_action.value();

    // NOTE: if you're following along the with video, this was implemented
    // incorrectly at the time I made it, due to a bug in <ActionForm/> that
    // was not resetting input. This is how it should have been implemented
    // all along, which would also have fixed the bug at 49:24!
    let prefers_dark = move || {
        println!("Rerunning prefers_dark()");
        match (input(), value()) {
            // if there's some current input, use that optimistically
            (Some(submission), _) => submission.prefers_dark,
            // otherwise, if there was a previous value confirmed by server, use that
            (_, Some(Ok(value))) => value,
            // otherwise, use the initial value
            _ => initial,
        }
    };
    println!("PREFERS DARK IS {:#?}", prefers_dark());
    // Set parent signal provided by context to value of prefers_dark
    let color_scheme = use_context::<ColorScheme>(cx).expect("Failed to find color scheme signal!");
    color_scheme.0.set(prefers_dark());
    // let color_scheme = move || {
    //     if prefers_dark() {
    //       //  "dark".to_string()
    //        true
    //     } else {
    //         //"light".to_string()
    //         false
    //     }
    // };

    view! { cx,
     //   <Meta
       //     name="color-scheme"
         //   content=color_scheme
        // />
        <ActionForm action=toggle_dark_mode_action>
            <input
                type="hidden"
                name="prefers_dark"
                value=move || (!prefers_dark()).to_string()
            />
            <input
                type="submit"
                value=move || {
                    if prefers_dark() {
                        "Switch to Light Mode"
                    } else {
                        "Switch to Dark Mode"
                    }
                }
            />
        </ActionForm>
    }
}
