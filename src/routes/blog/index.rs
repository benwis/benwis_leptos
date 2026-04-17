use crate::{
    functions::post::{DeletePost, PostQuery, get_post_count, get_posts_paginated},
    providers::AuthContext,
};
use leptos::prelude::*;
use leptos::{
    IntoView, component,
    context::use_context,
    reactive::owner::StoredValue,
    server::ServerAction,
    tachys::either::{Either, EitherOf3},
    view,
};
use leptos_meta::*;
use leptos_router::hooks::use_query;

#[component]
pub fn Blog() -> impl IntoView {
    let delete_post = ServerAction::<DeletePost>::new();

    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    let query = use_query::<PostQuery>();
    let page = move || query.read().as_ref().ok().and_then(|q| q.p).unwrap_or(1);

    let posts = Resource::new(
        move || (page(), delete_post.version().get()),
        move |(p, _)| get_posts_paginated(p, 5),
    );
    let total = Resource::new(move || (), move |_| get_post_count());

    let posts_view = move || {
        Suspend::new(async move {
            match posts.await {
                Err(e) => EitherOf3::A(
                    view! { <pre class="error">"Server Error: " {e.to_string()}</pre> },
                ),
                Ok(posts) => {
                    if posts.is_empty() {
                        EitherOf3::B(view! { <p>"No posts were found."</p> })
                    } else {
                        let cards = posts
                            .into_iter()
                            .map(move |post| {
                                let post_slug: StoredValue<String> =
                                    StoredValue::new(post.slug.clone());
                                let post_id = post.id;
                                let hero = post.hero.clone().unwrap_or_default();
                                view! {
                                    <a class="post-card" href=format!("/posts/{}", post.slug)>
                                        {(!hero.is_empty()).then(|| {
                                            view! { <img class="post-card__image" src=hero.clone() /> }
                                        })}
                                        <div class="post-card__text">
                                            <h2 class="post-card__heading">{post.title}</h2>
                                            <p class="post-card__meta">{post.created_at_pretty}</p>
                                            <p class="post-card__excerpt">{post.excerpt}</p>
                                        </div>
                                        <Transition fallback=|| ()>
                                            {move || Suspend::new(async move {
                                                match auth_context.user.await {
                                                    Ok(Some(_)) => Either::Left(view! {
                                                        <div class="post-card__admin">
                                                            <a href=format!(
                                                                "/posts/{}/edit",
                                                                post_slug.get_value(),
                                                            )>"Edit Post"</a>
                                                            <ActionForm action=delete_post>
                                                                <input type="hidden" name="id" value=post_id />
                                                                <input type="submit" value="Delete Post" />
                                                            </ActionForm>
                                                        </div>
                                                    }),
                                                    _ => Either::Right(()),
                                                }
                                            })}
                                        </Transition>
                                    </a>
                                }
                            })
                            .collect::<Vec<_>>();
                        EitherOf3::C(view! { <>{cards}</> })
                    }
                }
            }
        })
    };

    let pagination_numbers = move || {
        Suspend::new(async move {
            let count = total.await.unwrap_or(0);
            let last = if count % 5 == 0 {
                count / 5
            } else {
                count / 5 + 1
            };
            let current = page();
            let prev_link = (current > 1).then(|| {
                view! {
                    <a class="archive__pagination-link" href=format!("/posts?p={}", current - 1)>
                        "← "{current - 1}
                    </a>
                }
            });
            let next_link = (current < last).then(|| {
                view! {
                    <a class="archive__pagination-link" href=format!("/posts?p={}", current + 1)>
                        {current + 1}" →"
                    </a>
                }
            });
            let numbers = (1..=last).map(|i| view! {
                <a class="archive__pagination-page-link" href=format!("/posts?p={i}")>{i}</a>
            }).collect::<Vec<_>>();
            view! {
                <div class="archive__pagination">{prev_link}</div>
                <div id="archive__footer-numbers">{numbers}</div>
                <div class="archive__pagination">{next_link}</div>
            }
        })
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

        <div id="archive">
            <div id="archive-background"></div>
            <div id="archive__header">
                <h1 id="archive__heading">"Blog"</h1>
            </div>
            <div id="archive__posts">
                <Suspense fallback=|| view! { <p>"Loading..."</p> }>{posts_view}</Suspense>
            </div>
            <div id="archive__footer">
                <Suspense fallback=|| ()>{pagination_numbers}</Suspense>
            </div>
        </div>
    }
}
