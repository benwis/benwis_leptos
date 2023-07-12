use crate::components::{FeatureCard};
use crate::functions::post::{get_some_posts_meta, AddPost, DeletePost, UpdatePost};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    let add_post = create_server_multi_action::<AddPost>(cx);
    let update_post = create_server_action::<UpdatePost>(cx);
    let delete_post = create_server_action::<DeletePost>(cx);

    // list of posts is loaded from the server in reaction to changes
    let posts_meta = create_resource(
        cx,
        move || (add_post.version().get(), update_post.version().get(), delete_post.version().get()),
        move |_| get_some_posts_meta(cx),
    );

    view! { cx,
        <Meta property="og:title" content="benwis"/>
        <Title text="benwis"/>
        <Meta name="description" content="Ben Wishovich's personal website"/>
        <Meta property="og:description" content="Ben Wishovich's personal website"/>
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>

        <div class="flex w-9/12 flex-col-reverse items-start sm:flex-row">
            <div class="flex flex-col pt-20 mx-auto">
                <h1 class="mb-3 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
                    "I am"
                    <span class="relative ml-2 inline-block before:absolute before:-inset-1 before:block before:rounded-lg dark:bg-gray-900 before:py-8">
                        <span class="brand relative skew-y-3 py-4 px-2 text-7xl uppercase text-yellow-400 dark:text-yellow-400">
                            "BENWIS"
                        </span>
                    </span>
                </h1>
                <h2 class="mb-4 text-gray-700 dark:text-gray-200">
                    "Software Engineer, Full Stack Web Developer, Runner." <br/>
                    <span class="font-semibold">
                        "Rust, Typescript, WASM, Python, React, Svelte"
                    </span>
                </h2>
                <p class="mb-16 text-gray-600 text-center">
                    <a
                        href="https://github.com/benwis"
                        class="rounded font-bold text-yellow-400 inline-flex items-center px-0 py-2 border border-transparent font-medium rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-stone-500 hover:text-yellow-500"
                    >
                        "Check out my work!"
                    </a>
                </p>
            </div>
        </div>
        <section class="mb-16 w-9/12 flex flex-col">
            <div class="mx-auto">
                <h3 class="mb-6 text-2xl font-bold tracking-tight text-black dark:text-white md:text-4xl">
                    "Recent Posts"
                </h3>
                <div class="flex flex-col gap-6 md:flex-row">
                    <Transition fallback=move || {
                        view! { cx, <p>"Loading..."</p> }
                    }>
                        {move || {
                            let posts_meta = {
                                move || {
                                    posts_meta
                                        .read(cx)
                                        .map(move |posts_meta| match posts_meta {
                                            Err(e) => {
                                                vec![
                                                    view! { cx, < pre class = "error" > "Server Error: " { e
                                                    .to_string() } </ pre > } .into_view(cx)
                                                ]
                                            }
                                            Ok(posts_meta) => {
                                                if posts_meta.is_empty() {
                                                    vec![
                                                        view! { cx, < p class = "text-black dark:text-white" >
                                                        "No posts were found." </ p > } .into_view(cx)
                                                    ]
                                                } else {
                                                    posts_meta
                                                        .into_iter()
                                                        .map(move |post_meta| {
                                                            view! { cx, <FeatureCard href={post_meta.slug} title={post_meta.title} date={post_meta.created_at_pretty}/> }
                                                                .into_view(cx)
                                                        })
                                                        .collect::<Vec<_>>()
                                                }
                                            }
                                        })
                                        .unwrap_or_default()
                                }
                            };
                            posts_meta.into_view(cx)
                        }}
                    </Transition>
                </div>
                <a
                    class="mt-8 flex h-6 rounded-lg leading-7 text-gray-600 transition-all dark:text-gray-400 dark:hover:text-gray-200"
                    href="posts"   
                >
                    "See more posts"
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="ml-1 h-6 w-6"
                    >
                        <path
                            stroke="currentColor"
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth="2"
                            d="M17.5 12h-15m11.667-4l3.333 4-3.333-4zm3.333 4l-3.333 4 3.333-4z"
                        ></path>
                    </svg>
                </a>
            </div>
        </section>
    }
}
