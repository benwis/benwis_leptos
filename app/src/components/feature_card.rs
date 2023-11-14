use leptos::*;

#[component]
pub fn FeatureCard(title: String, href: String, date: String) -> impl IntoView {
    view! {
        <a
            class="w-full transform rounded-xl bg-gradient-to-r from-yellow-400 via-rose-400 to-cyan-500 p-1 transition-all hover:scale-[1.01] md:w-1/3"
            href=format!("/posts/{href}")
        >
            <div class="flex h-full flex-col justify-between rounded-lg bg-white p-4 dark:bg-gray-900">
                <div class="flex flex-col justify-between md:flex-row">
                    <h4 class="mb-6 w-full text-lg font-medium tracking-tight text-gray-900 dark:text-gray-100 sm:mb-10 md:text-lg">
                        {title}
                    </h4>
                </div>
                <div class="capsize flex items-center text-gray-800 dark:text-gray-200">{date}</div>
            </div>
        </a>
    }
}
