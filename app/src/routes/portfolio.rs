use crate::components::PortfolioCard;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
        <Meta property="og:title" content="My Portfolio"/>
        <Title text="My Portfolio"/>
        <Meta name="description" content="A collection of things I've built or helped build."/>
        <Meta
            property="og:description"
            content="A collection of things I've built or helped build."
        />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
        <article class="mx-auto mb-16 flex  w-full max-w-2xl flex-col items-start justify-center px-4 sm:px-8">
            <h1 class="mb-2 text-3xl font-bold tracking-tight text-black dark:text-white md:text-5xl ">
                "Portfolio"
            </h1>
            <div
                sty
                class="bg mt-2 flex w-full justify-between sm:flex-col sm:items-start md:flex-row md:items-center"
            >
                <p class="min-w-32 flex items-center text-sm text-gray-600 dark:text-gray-400 md:mt-0"></p>
            </div>
            <div class="-mx-4 my-2 flex h-1 w-[100vw] bg-gradient-to-r from-yellow-400 via-rose-400 to-cyan-500 sm:mx-0 sm:w-full"></div>
            <div class="mt-16 mb-32 w-full max-w-none dark:prose-invert">
                <main
                    role="list"
                    class="space-y-12 no-underline sm:grid sm:grid-cols-1 sm:gap-x-6 sm:gap-y-12 sm:space-y-0 lg:grid-cols-2 lg:gap-x-8"
                >
                    <PortfolioCard
                        heading="Praxis Cycles".to_string()
                        sub_heading="Ecommerce Store".to_string()
                        href="https://praxiscycles.com".to_string()
                        img="/img/praxiscycles_square.png".to_string()
                        description="Praxis Cycles is a bike parts supplier that does over a million dollars in annual sales. I redesigned the site to a media heavy layout, optimized image distribution, and massively decreased page load time"
                            .to_string()
                    />
                    <PortfolioCard
                        heading="Praxis OEM Site".to_string()
                        sub_heading="B2B Sales Site".to_string()
                        href="https://oem.praxiscycles.com".to_string()
                        img="/img/praxis_oem_square.png".to_string()
                        description="Praxis' OEM site is a login only site that lists their hundreds of available parts for dealers to purchase. I built the frontend using Remix, React, and TailwindCSS. The backend is a custom rust GraphQL server linked to Airtable and Sanity for data entry."
                            .to_string()
                    />
                </main>
            </div>
        </article>
    }
}
