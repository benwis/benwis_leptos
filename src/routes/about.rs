use leptos::*;
use leptos_meta::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Meta property="og:title" content="About Me"/>
        <Title text="About Me"/>
        <Meta name="description" content="A page describing me"/>
        <Meta property="og:description" content="A page describing me"/>
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>

        <div class="mx-auto grid max-w-2xl grid-cols-1 items-start justify-center gap-y-20 border-gray-200 px-4 pb-16 dark:border-gray-900 sm:px-8 md:grid-cols-2 md:gap-x-20">
            <section>
                <div>
                    <div class="flex max-w-2xl flex-col items-start justify-center border-gray-200 dark:border-gray-900">
                        <div class="flex flex-col-reverse items-start sm:flex-row">
                            <div class="flex w-full flex-col">
                                <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white md:text-5xl">
                                    "I am"
                                    <span class="relative ml-2 inline-block before:absolute before:-inset-1 before:block before:rounded-lg before:py-8">
                                        <span class="brand relative py-8 text-5xl uppercase text-yellow-400">
                                            "BENWIS"
                                        </span>
                                    </span>
                                </h1>
                            </div>
                        </div>
                    </div>
                    <div class="mb-6 flex max-w-2xl flex-col items-start justify-center border-gray-200 dark:border-gray-900">
                        <div class="flex flex-col-reverse items-start sm:flex-row">
                            <div class="flex w-full flex-col">
                                <h1 class="text-2xl font-bold tracking-tight text-black dark:text-white md:text-2xl">
                                    "AKA"
                                    <span class="relative ml-2 inline-block before:absolute before:-inset-1 before:block before:rounded-lg">
                                        <span class="brand relative skew-y-3 py-8 text-4xl uppercase text-yellow-400 dark:text-yellow-400">
                                            "Ben Wishovich"
                                        </span>
                                    </span>
                                </h1>
                            </div>
                        </div>
                    </div>
                </div>
                <p class="text-black dark:text-white">
                    "I'm a Software Engineer and Full Stack Web Developer, living and working in the SF Bay Area. I
                                                                                                                                                                graduated from San Jose State with a degree in Industrial and Systems Engineering, and then
                                                                                                                                                                made the jump into software engineering and web development."
                </p>
                <p class="text-black dark:text-white">
                    "I've been coding and building things since High School, and have helped build a variety of
                                                                                                                                                                projects. Everything from mapping software for UAVs, motor controllers for automated vending
                                                                                                                                                                machines, to electric motorcycle diagnostic software."
                </p>
                <p class="text-black dark:text-white">
                    "I build web experiences using Python, TypeScript, and Rust along with React, Remix, and Svelte. Currently exploring the boundaries
                                                                                                                                                                of web development with WASM, GraphQL, Remix, and Svelte."
                </p>
            </section>
            <section>
                <img
                    class="rounded"
                    src="/img/ben_catcarbon.png"
                    alt="A white guy with blue eyes, dark hair, and glasses. Kinda looks like Harry Potter. Describing yourself is hard"
                />
                <address class="my-4 w-full transform rounded-xl bg-gradient-to-r from-yellow-400 via-rose-400 to-cyan-500 p-1 transition-all hover:scale-[1.01]">
                    <section class="flex flex-col rounded-lg bg-white p-4 dark:bg-gray-900">
                        <h3 class="text-xl text-black dark:text-white">"Contact Me:"</h3>
                        <a href="mailto:ben@benw.is" class="text-black dark:text-white">
                            "Email"
                        </a>
                        <a href="https://twitter.com/iambenwis" class="text-black dark:text-white">
                            "Twitter"
                        </a>
                    </section>
                </address>
            </section>
        </div>
    }
}
