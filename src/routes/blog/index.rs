use crate::functions::post::get_posts;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Blog() -> impl IntoView {
    // list of posts is loaded from the server in reaction to changes
    let posts = create_resource(move || {}, move |_| get_posts(None));

    view! {
        <Meta property="og:title" content="benwis Blog"/>
        <Title text="benwis Blog"/>
        <Meta
            name="description"
            content="The potentially misguided ramblings of a Rust developer flailing around on the web"
        />
        <Meta
            property="og:description"
            content="The potentially misguided ramblings of a Rust developer flailing around on the web"
        />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                let existing_posts = {
                    move || {
                        posts
                            .get()
                            .map(move |posts| match posts {
                                Err(e) => {
                                    vec![
                                        view! { < pre class = "error" > "Server Error: " { e
                                        .to_string() } </ pre > } .into_any()
                                    ]
                                }
                                Ok(Err(e)) => {
                                    vec![
                                        view! { < pre class = "error" > "Error: " { e.to_string() }
                                        </ pre > } .into_any()
                                    ]
                                }
                                Ok(Ok(posts)) => {
                                    if posts.posts.len() == 0 {
                                        vec![
                                            view! { < p class = "text-black dark:text-white" >
                                            "No posts were found." </ p > } .into_any()
                                        ]
                                    } else {
                                        posts
                                            .posts
                                            .into_values()
                                            .filter(|post| { post.published })
                                            .map(move |post| {
                                                view! {
                                                    <section>
                                                        <a
                                                            href=format!("/posts/{}", post.slug)
                                                            class="no-underline hover:underline hover:decoration-yellow-400"
                                                        >
                                                            <li class="mb-8 text-lg">
                                                                <div class="inline-flex justify-between w-full">
                                                                    <h4 class="text-lg font-medium md:text-xl text-black dark:text-white">
                                                                        {post.title}
                                                                    </h4>
                                                                    <p class=" text-left text-gray-500 dark:text-gray-400 md:mb-0 md:text-right">
                                                                        {post.created_at.to_string()}
                                                                    </p>
                                                                </div>
                                                                <p class="text-gray-500">{post.excerpt}</p>
                                                            </li>
                                                        </a>
                                                    </section>
                                                }
                                                    .into_any()
                                            })
                                            .collect::<Vec<_>>()
                                    }
                                }
                            })
                            .unwrap_or_default()
                    }
                };
                view! {
                    <div class="dark:text-white w-full max-w-5xl px-12">
                        <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
                            "Posts"
                        </h1>
                        <ul>{existing_posts}</ul>
                    </div>
                }
            }}

        </Transition>
    }
}
