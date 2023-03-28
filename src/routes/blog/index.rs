use crate::{
    functions::post::{get_posts, AddPost, DeletePost, UpdatePost},
    providers::AuthContext,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
    let add_post = create_server_multi_action::<AddPost>(cx);
    let update_post = create_server_action::<UpdatePost>(cx);
    let delete_post = create_server_action::<DeletePost>(cx);

    let submissions = add_post.submissions();

    // list of posts is loaded from the server in reaction to changes
    let posts = create_resource(
        cx,
        move || {
            (
                add_post.version().get(),
                update_post.version().get(),
                delete_post.version().get(),
            )
        },
        move |_| get_posts(cx),
    );

    let auth_context = use_context::<AuthContext>(cx).expect("Failed to get AuthContext");

    view! { cx,
        <Transition fallback=move || {
            view! { cx, <p>"Loading..."</p> }
        }>
            {move || {
                let existing_posts = {
                    move || {
                        posts
                            .read(cx)
                            .map(move |posts| match posts {
                                Err(e) => {
                                    vec![
                                        view! { cx, < pre class = "error" > "Server Error: " { e
                                        .to_string() } </ pre > } .into_any()
                                    ]
                                }
                                Ok(posts) => {
                                    if posts.is_empty() {
                                        vec![
                                            view! { cx, < p class = "text-black dark:text-white" >
                                            "No posts were found." </ p > } .into_any()
                                        ]
                                    } else {
                                        posts
                                            .into_iter()
                                            .filter(|post| {
                                                log!("{}", post.published);
                                                post.published
                                            })
                                            .map(move |post| {
                                                let post_slug: StoredValue<String> = store_value(
                                                    cx,
                                                    post.slug.clone(),
                                                );
                                                view! { cx,
                                                    <section>
                                                    <Meta property="og:title" content="benwis Blog"/>
                                                    <Meta name="title" content="benwis Blog"/>
                                                    <Title text="benwis Blog"/>
                                                    <Meta name="description" content="The potentially misguided ramblings of a Rust developer flailing around on the web"/>
                                                    <Meta property="og:description" content="The potentially misguided ramblings of a Rust developer flailing around on the web"/>
                                                    <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
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
                                                        <Transition fallback=move || ()>
                                                            {move || {
                                                                let user = move || match auth_context.user.read(cx) {
                                                                    Some(Ok(Some(user))) => Some(user),
                                                                    Some(Ok(None)) => None,
                                                                    Some(Err(_)) => None,
                                                                    None => None,
                                                                };
                                                                view! { cx,
                                                                    <Show when=move || user().is_some() fallback=|_| ()>
                                                                        <A href={format!("{}/edit", post_slug.get_value())}>"Edit Post"</A>
                                                                        <ActionForm action=delete_post>
                                                                            <input type="hidden" name="id" value={post.id}/>
                                                                            <input type="submit" value="Delete Post"/>
                                                                        </ActionForm>
                                                                    </Show>
                                                                }
                                                            }}
                                                        </Transition>
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
                let pending_posts = move || {
                    submissions
                        .get()
                        .into_iter()
                        .filter(|submission| submission.pending().get())
                        .map(|submission| {
                            view! { cx, <li class="pending">{move || submission.input.get().map(|data| data.title)}</li> }
                        })
                        .collect::<Vec<_>>()
                };
                view! { cx,
                    <div class="dark:text-white w-full max-w-5xl px-12">
                        <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
                            "Posts"
                        </h1>
                        <ul>{pending_posts} {existing_posts}</ul>
                    </div>
                }
            }}
        </Transition>
    }
}
