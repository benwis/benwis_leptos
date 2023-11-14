use leptos::*;

pub struct FooterLink<'a> {
    name: &'a str,
    href: &'a str,
    icon: &'a str,
    rel: Option<&'a str>,
}

#[component]
pub fn Footer() -> impl IntoView {
    let mut navigation: Vec<FooterLink> = Vec::new();

    navigation.push(FooterLink{
        name: "RSS",
        href: "/rss.xml",
        icon: r#"
            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-6 h-6">
                <path strokeLinecap="round" strokeLinejoin="round" d="M12.75 19.5v-.75a7.5 7.5 0 00-7.5-7.5H4.5m0-6.75h.75c7.87 0 14.25 6.38 14.25 14.25v.75M6 18.75a.75.75 0 11-1.5 0 .75.75 0 011.5 0z" />
            </svg>
        "#,
        rel: Some("external")
    });
    navigation.push(FooterLink{
        name: "Twitter",
        href: "https://twitter.com/iambenwis",
        icon: r#"
            <svg fill="currentColor" viewBox="0 0 24 24" class="w-6 h-6">
                <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84" />
            </svg>
        "#,
        rel: None
    });
    navigation.push(FooterLink{
        name: "Mastodon",
        href: "https://hachyderm.io/@benwis",
        icon: r#"
            <svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="w-6 h-6">
                <path d="M433 179.11c0-97.2-63.71-125.7-63.71-125.7-62.52-28.7-228.56-28.4-290.48 0 0 0-63.72 28.5-63.72 125.7 0 115.7-6.6 259.4 105.63 289.1 40.51 10.7 75.32 13 103.33 11.4 50.81-2.8 79.32-18.1 79.32-18.1l-1.7-36.9s-36.31 11.4-77.12 10.1c-40.41-1.4-83-4.4-89.63-54a102.54 102.54 0 0 1-.9-13.9c85.63 20.9 158.65 9.1 178.75 6.7 56.12-6.7 105-41.3 111.23-72.9 9.8-49.8 9-121.5 9-121.5zm-75.12 125.2h-46.63v-114.2c0-49.7-64-51.6-64 6.9v62.5h-46.33V197c0-58.5-64-56.6-64-6.9v114.2H90.19c0-122.1-5.2-147.9 18.41-175 25.9-28.9 79.82-30.8 103.83 6.1l11.6 19.5 11.6-19.5c24.11-37.1 78.12-34.8 103.83-6.1 23.71 27.3 18.4 53 18.4 175z"/>
            </svg>
        "#,
        rel: Some("me noreferrer"),
    });
    navigation.push(FooterLink{
        name: "Github",
        href: "https://github.com/benwis",
        icon: r#"
            <svg fill="currentColor" viewBox="0 0 24 24" class="w-6 h-6">
                <path
                fillRule="evenodd"
                d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                clipRule="evenodd"
                />
            </svg>
        "#,
        rel: None
    });

    view! {
        <footer class="bg-white dark:bg-gray-900 mt-auto">
            <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 md:flex md:items-center md:justify-between lg:px-8">
                <div class="flex justify-center space-x-6 md:order-2">
                    {navigation
                        .into_iter()
                        .map(|link| {
                            view! {
                                <a
                                    key=link.name
                                    href=link.href
                                    rel=link.rel
                                    class="text-gray-400 dark:text-white hover:text-gray-500"
                                >
                                    <span class="sr-only">{link.name}</span>
                                    <div inner_html=link.icon></div>
                                </a>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                <div class="mt-8 md:mt-0 md:order-1">
                    <p class="text-center text-base text-gray-400 dark:text-white">
                        "© 2023 Ben Wishovich | Built with "
                        <a href="https://leptos.dev">"Leptos"</a> <span>" | v3"</span>
                    </p>
                </div>
            </div>
        </footer>
    }
}
