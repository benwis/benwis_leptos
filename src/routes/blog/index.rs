use crate::{
    functions::post::{get_posts, AddPost, DeletePost, UpdatePost},
    providers::AuthContext,
};
use leptos::{
    component,
    context::use_context,
    error::ServerFnError,
    reactive_graph::owner::StoredValue,
    server::{self, ServerAction},
    tachys::either::{Either, EitherOf3},
    view, IntoView,
};
use leptos::{prelude::*, server::ArcResource};
use leptos::{reactive_graph::owner::Owner, server::Resource};
use leptos_meta::*;

#[component]
pub fn Blog() -> impl IntoView {
    // TODO
    let add_post = ServerAction::<AddPost>::new();
    let update_post = ServerAction::<UpdatePost>::new();
    let delete_post = ServerAction::<DeletePost>::new();

    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    let posts = Resource::new_serde(|| (), |_| get_posts());

    let posts_view = {
        async move {
            match posts.await {
                Err(e) => EitherOf3::A(view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }),
                Ok(mut posts) => {
                    posts.sort_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());
                    if posts.is_empty() {
                        EitherOf3::B(view! {  < p class = "text-black dark:text-white" >
                            "No posts were found." </ p > })
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
                                    /*<Transition fallback=move || ()>
                                      {move || {
                                      let user = move || match auth_context.user.read() {
                                      Some(Ok(Some(user))) => Some(user),
                                      Some(Ok(None)) => None,
                                      Some(Err(_)) => None,
                                      None => None,
                                      };
                                      view! {
                                      <Show when=move || user().is_some() fallback=|| ()>
                                      <A href={format!("{}/edit", post_slug.get_value())}>"Edit Post"</A>
                                      <ActionForm action=delete_post>
                                      <input type="hidden" name="id" value={post.id}/>
                                      <input type="submit" value="Delete Post"/>
                                      </ActionForm>
                                      </Show>
                                      }
                                      }}
                                      </Transition>*/
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
        <Meta property="og:title" content="benwis Blog"/>
        <Title text="benwis Blog"/>
        <Meta name="description" content="The potentially misguided ramblings of a Rust developer flailing around on the web"/>
        <Meta property="og:description" content="The potentially misguided ramblings of a Rust developer flailing around on the web"/>
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
        <pre>{posts_view}</pre>
        // TOOD
        /*<Transition fallback=move || {
            view! {  <p>"Loading..."</p> }
        }>
            {move || {
                let existing_posts = {
                    move || {
                        posts
                            .read()
                            .map(move |posts| match posts {
                                Err(e) => {
                                    vec![
                                        view! {  < pre class = "error" > "Server Error: " { e
                                        .to_string() } </ pre > } .into_any()
                                    ]
                                }
                                Ok(mut posts) => {
                                    // Reverse the order of the posts
                                    posts.sort_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());
                                    if posts.is_empty() {
                                        vec![
                                            view! {  < p class = "text-black dark:text-white" >
                                            "No posts were found." </ p > } .into_any()
                                        ]
                                    } else {
                                        posts
                                            .into_iter()
                                            .filter(|post| {
                                                post.published
                                            })
                                            .map(move |post| {
                                                let post_slug: StoredValue<String> = store_value(

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
                                                        <Transition fallback=move || ()>
                                                            {move || {
                                                                let user = move || match auth_context.user.read() {
                                                                    Some(Ok(Some(user))) => Some(user),
                                                                    Some(Ok(None)) => None,
                                                                    Some(Err(_)) => None,
                                                                    None => None,
                                                                };
                                                                view! {
                                                                    <Show when=move || user().is_some() fallback=|| ()>
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
                            view! {  <li class="pending">{move || submission.input.get().map(|data| data.title)}</li> }
                        })
                        .collect::<Vec<_>>()
                };
                view! {
                    <div class="dark:text-white w-full max-w-5xl px-12">
                        <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
                            "Posts"
                        </h1>
                        <ul>{pending_posts} {existing_posts}</ul>
                    </div>
                }
            }}
        </Transition>*/
    }
}
