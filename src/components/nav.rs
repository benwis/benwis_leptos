use leptos::*;

use crate::providers::AuthContext;
use crate::components::{DarkModeToggle, DarkModeToggleProps};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView{
    let auth_context = use_context::<AuthContext>(cx).expect("Failed to get AuthContext");
    
    view! { cx,
        <nav class="relative bg-white dark:bg-gray-900 text-gray-700 dark:text-white  flex items-center justify-between max-w-7xl p-4">
            <div class="flex space-x-10 items-center">
                <a href="/">"Home"</a>
                <a href="/posts">"Blog"</a>
                <a href="/about">"About Me"</a>
                <a href="/portfolio">"Portfolio"</a>
            </div>
            <div class="flex space-x-10 items-center">
                <DarkModeToggle/>
                <Transition fallback=move || {
                    view! { cx, <p>"Loading..."</p> }
                }>
                    {
                        let user = move || match auth_context.user.read(cx) {
                            Some(Ok(Some(user))) => Some(user),
                            Some(Ok(None)) => None,
                            Some(Err(_)) => None,
                            None => None,
                        };
                        view! { cx,
                            <Show
                                when=move || user().is_some()
                                fallback=|cx| {
                                    view! { cx, <a href="/signup">"Signup"</a> }
                                }
                            >
                                {|| ()}
                            </Show>
                            <Show
                                when=move || user().is_some()
                                fallback=|cx| {
                                    view! { cx, <a href="/login">"Login"</a> }
                                }
                            >
                                <a href="/logout">"Logout"</a>
                            </Show>
                        }
                    }
                </Transition>
            </div>
        </nav>
    }
}