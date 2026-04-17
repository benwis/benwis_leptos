use crate::providers::ColorScheme;
use leptos::prelude::*;
use leptos::{IntoView, component, context::use_context, view};

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let color_scheme = use_context::<ColorScheme>().expect("Failed to find ColorSchemeProvider");

    view! {
        <li id="site-nav__dark-mode-toggle">
            <ActionForm action=color_scheme.action>
                <input
                    type="hidden"
                    name="prefers_dark"
                    value=move || (!(color_scheme.prefers_dark).get()).to_string()
                />
                <button
                    id="site-nav__dark-mode-toggle-trigger"
                    type="submit"
                    aria-label="toggle between dark and light mode"
                    value=move || {
                        if (color_scheme.prefers_dark).get() { "dark" } else { "light" }
                    }
                    inner_html=move || {
                        if (color_scheme.prefers_dark).get() {
                            r#"<svg
                                    id="site-nav__dark-mode-toggle-light"
                                    fill="currentColor"
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                >
                                    <path d="M4.069 13h-4.069v-2h4.069c-.041.328-.069.661-.069 1s.028.672.069 1zm3.034-7.312l-2.881-2.881-1.414 1.414 2.881 2.881c.411-.529.885-1.003 1.414-1.414zm11.209 1.414l2.881-2.881-1.414-1.414-2.881 2.881c.528.411 1.002.886 1.414 1.414zm-6.312-3.102c.339 0 .672.028 1 .069v-4.069h-2v4.069c.328-.041.661-.069 1-.069zm0 16c-.339 0-.672-.028-1-.069v4.069h2v-4.069c-.328.041-.661.069-1 .069zm7.931-9c.041.328.069.661.069 1s-.028.672-.069 1h4.069v-2h-4.069zm-3.033 7.312l2.88 2.88 1.415-1.414-2.88-2.88c-.412.528-.886 1.002-1.415 1.414zm-11.21-1.415l-2.88 2.88 1.414 1.414 2.88-2.88c-.528-.411-1.003-.885-1.414-1.414zm2.312-4.897c0 2.206 1.794 4 4 4s4-1.794 4-4-1.794-4-4-4-4 1.794-4 4zm10 0c0 3.314-2.686 6-6 6s-6-2.686-6-6 2.686-6 6-6 6 2.686 6 6z"></path>
                                </svg>"#
                        } else {
                            r#"<svg
                                    id="site-nav__dark-mode-toggle-dark"
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                >
                                    <path d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm.685 21.965c3.205-2.154 5.315-5.813 5.315-9.965s-2.11-7.811-5.315-9.965c5.202.353 9.315 4.673 9.315 9.965s-4.113 9.612-9.315 9.965z"></path>
                                </svg>"#
                        }
                    }
                ></button>
            </ActionForm>
        </li>
    }
}
