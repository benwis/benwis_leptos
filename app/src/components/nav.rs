use crate::components::DarkModeToggle;
use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="top-nav bg-white dark:bg-gray-900 text-gray-700 dark:text-white">
            <div class="text-2xl">
                <a href="/">
                    <h3 class="text-2xl dark:text-white text-base">"BENWIS"</h3>
                </a>
            </div>
            <input id="menu-toggle" type="checkbox"/>
            <label class="menu-button-container" for="menu-toggle">
                <div class="menu-button"></div>
            </label>
            <ul class="menu items-center z-10">
                <li class="items-center">
                    <a href="/posts">"Blog"</a>
                </li>
                <li class="items-center">
                    <a href="/about">"About Me"</a>
                </li>
                <li class="items-center">
                    <a href="/portfolio">"Portfolio"</a>
                </li>
                <DarkModeToggle/>
            </ul>
        </nav>
    }
}
