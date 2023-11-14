use leptos::*;

#[component]
pub fn PortfolioCard(
    img: String,
    href: String,
    heading: String,
    sub_heading: String,
    description: String,
) -> impl IntoView {
    view! {
        <section>
            <a href=href>
                <div class="space-y-4">
                    <div class="aspect-w-3 aspect-h-2">
                        <img class="object-cover h-200px shadow-lg rounded-lg" src=img alt=""/>
                    </div>
                    <div class="space-y-2">
                        <div class="text-lg leading-6 font-medium space-y-1">
                            <h3 class="dark:text-white text-black">{heading}</h3>
                            <p class="text-indigo-600">{sub_heading}</p>
                        </div>
                        <div role="list" class="flex space-x-5 no-underline">
                            <p class="dark:text-white text-black">{description}</p>
                        </div>
                    </div>
                </div>
            </a>
        </section>
    }
}
