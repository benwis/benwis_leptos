use leptos::*;
use leptos_meta::*;

use crate::providers::color_scheme::{ColorScheme};
use crate::components::{Nav, NavProps, Footer, FooterProps};

#[component]
pub fn Default(cx: Scope, children: Children) -> impl IntoView{
    let color_scheme = use_context::<ColorScheme>(cx).expect("Failed to find ColorScheme");

    view! { cx,
        <Html class=move || {
            let classes = "h-full";
            let theme = match color_scheme.prefers_dark.get() {
                true => "dark",
                false => "",
            };
            format!("{} {}", classes, theme)
        }/>
        <Body class="h-screen bg-white dark:bg-gray-900 max-w-5xl mx-auto flex flex-col"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="styles/output.css"/>
        <Nav/>
        <main class="mx-auto flex w-full flex-col items-center justify-center border-gray-200 px-4 pb-16 md:pt-4 dark:border-gray-900 sm:px-8">
            {children(cx)}
        </main>
        <Footer/>
    }
   
}