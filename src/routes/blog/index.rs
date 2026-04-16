use crate::{
    functions::post::{get_posts, AddPost, DeletePost, UpdatePost},
    providers::AuthContext,
};
use leptos::{
    component, context::use_context, reactive::owner::StoredValue, server::ServerAction, tachys::either::EitherOf3, view, IntoView
};
use leptos_meta::*;
use leptos::prelude::*;

#[component]
pub fn Blog() -> impl IntoView {
    // TODO
    let add_post = ServerAction::<AddPost>::new();
    let update_post = ServerAction::<UpdatePost>::new();
    let delete_post = ServerAction::<DeletePost>::new();

    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    let posts = Resource::new(|| (), |_| get_posts());

    let posts_view = {
        async move {
            match posts.await {
                Err(e) => EitherOf3::A(view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }),
                Ok(mut posts) => {
                    posts.sort_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());
                    if posts.is_empty() {
                        EitherOf3::B(view! { <p class="text-black dark:text-white">"No posts were found."</p> })
                    } else {
                        let posts = posts
                            .into_iter()
                            .filter(|post| {
                                post.published
                            })
                        .map(move |post| {
                            let post_slug: StoredValue<String> = StoredValue::new(
                                post.slug.clone(),
                                );
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
                                                    {post.created_at_pretty}
                                                </p>
                                            </div>
                                            <p class="text-gray-500">{post.excerpt}</p>
                                        </li>
                                    </a>

                                </section>
                            }
                        })
                        .collect::<Vec<_>>();
                        EitherOf3::C(posts)
                    }
                }
            }
        }
        .suspend()
        .with_fallback("Loading...")
    };

    view! {
        <Meta property="og:title" content="benwis Blog" />
        <Title text="benwis Blog" />
        <Meta
            name="description"
            content="The potentially misguided ramblings of a Rust developer flailing around on the web"
        />
        <Meta
            property="og:description"
            content="The potentially misguided ramblings of a Rust developer flailing around on the web"
        />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg" />
        <pre>{posts_view}</pre>
    }
}
