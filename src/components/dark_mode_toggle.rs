use crate::providers::ColorScheme;
use leptos::*;
use leptos_router::{ActionForm, ActionFormProps};

#[component]
pub fn DarkModeToggle(cx: Scope) -> impl IntoView {
    let color_scheme = use_context::<ColorScheme>(cx).expect("Failed to find ColorSchemeProvider");

    view! { cx,
        <ActionForm action=color_scheme.action>
            <input
                type="hidden"
                name="prefers_dark"
                value=move || (!(color_scheme.prefers_dark)()).to_string()
            />
            <input
                type="submit"
                value=move || {
                    if (color_scheme.prefers_dark)() {
                        "Switch to Light Mode"
                    } else {
                        "Switch to Dark Mode"
                    }
                }
            />
        </ActionForm>
    }
}
