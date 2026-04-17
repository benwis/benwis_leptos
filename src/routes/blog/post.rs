use crate::functions::post::get_post_with_siblings;
use crate::models::post::{self, PostTriad};
use leptos::either::EitherOf4;
use leptos::server::Resource;
use leptos::{IntoView, component};
use leptos::{prelude::*, view};
use leptos_meta::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let triad = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        get_post_with_siblings,
    );

    view! {
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            {move || {
                Suspend::new(async move {
                    match triad.await {
                        Ok(Ok(Some(triad))) => EitherOf4::A(view! { <PostContent triad=triad /> }),
                        Ok(Ok(None)) => EitherOf4::B(view! { <p>"Post Not Found"</p> }),
                        Ok(Err(_)) => EitherOf4::C(view! { <p>"Server Error"</p> }),
                        Err(_) => EitherOf4::D(view! { <p>"Server Fn Error"</p> }),
                    }
                })
            }}
        </Transition>
    }
}

#[component]
pub fn PostContent(triad: PostTriad) -> impl IntoView {
    let post = triad.post;
    let hero = post.hero.clone().unwrap_or_default();
    let hero_alt = post.hero_alt.clone().unwrap_or_default();
    let hero_caption = post.hero_caption.clone().unwrap_or_default();
    let excerpt = post.excerpt.clone().unwrap_or_default();
    let title = post.title.clone();
    let slug = post.slug.clone();
    let toc = post.toc.clone().unwrap_or_default();
    let has_hero = post.hero.is_some();

    view! {
        <Meta property="og:title" content=title.clone() />
        <Meta property="og:description" content=excerpt.clone() />
        <Meta property="og:site_name" content="benw.is" />
        <Meta property="og:locale" content="en-us" />
        <Meta property="og:type" content="article" />
        <Meta
            property="og:image"
            content=if has_hero { hero.clone() } else { "https://benw.is/img/ben_catcarbon.png".to_string() }
        />
        <Meta property="og:image:type" content="image/png" />
        <Meta property="og:url" content=format!("https://benw.is/posts/{slug}") />
        <Meta name="twitter:title" content=title.clone() />
        <Meta name="twitter:site" content="@iambenwis" />
        <Title text=title.clone() />
        <Meta
            name="twitter:card"
            content=if has_hero { "summary_large_image" } else { "summary" }
        />
        <Meta
            name="twitter:image"
            content=if has_hero { hero.clone() } else { "https://benw.is/img/ben_catcarbon.png".to_string() }
        />
        <Meta name="twitter:description" content=excerpt.clone() />
        <Meta name="description" content=excerpt.clone() />
        <Link rel="canonical" href=format!("https://benw.is/posts/{slug}") />

        {(post.preview || post.published)
            .then(move || {
                view! {
                    <div id="page">
                        <div id="page__header">
                            <h1 id="page__heading">{title}</h1>
                            <p id="page__meta">{post.created_at_pretty}</p>
                        </div>
                        <div id="page__layout">
                            <div id="page__body">
                                <div id="page__toc">
                                    <div id="page__toc-sticky">
                                        <div class="page__sidebar-section">
                                            <h2 class="page__sidebar-section-heading">"Contents"</h2>
                                            <div inner_html=toc></div>
                                        </div>
                                    </div>
                                </div>
                                <main id="page__content" class="content">
                                    {has_hero.then(|| view! {
                                        <div class="post__hero">
                                            <img id="post__image" src=hero.clone() alt=hero_alt.clone() />
                                            <caption>{hero_caption.clone()}</caption>
                                        </div>
                                        <hr />
                                    })}
                                    <div inner_html=post.html></div>
                                </main>
                            </div>
                            <div id="page__sidebar">
                                <div id="page__sidebar-sticky">
                                    {triad.previous.map(|prev| {
                                        let prev_hero = prev.hero.clone().unwrap_or_default();
                                        let prev_hero_alt = prev.hero_alt.clone().unwrap_or_default();
                                        view! {
                                            <div class="page__sidebar-section">
                                                <h2 class="page__sidebar-section-heading">"Previous"</h2>
                                                <a class="post-card" href=format!("/posts/{}", prev.slug)>
                                                    {(!prev_hero.is_empty()).then(|| view! {
                                                        <img class="post-card__image" src=prev_hero.clone() alt=prev_hero_alt.clone() />
                                                    })}
                                                    <div class="post-card__text">
                                                        <h2 class="post-card__heading">{prev.title}</h2>
                                                        <p class="post-card__meta">{prev.created_at_pretty}</p>
                                                        <p class="post-card__excerpt">{prev.excerpt}</p>
                                                    </div>
                                                </a>
                                            </div>
                                        }
                                    })}
                                    {triad.next.map(|next| {
                                        let next_hero = next.hero.clone().unwrap_or_default();
                                        let next_hero_alt = next.hero_alt.clone().unwrap_or_default();
                                        view! {
                                            <div class="page__sidebar-section">
                                                <h2 class="page__sidebar-section-heading">"Next"</h2>
                                                <a class="post-card" href=format!("/posts/{}", next.slug)>
                                                    {(!next_hero.is_empty()).then(|| view! {
                                                        <img class="post-card__image" src=next_hero.clone() alt=next_hero_alt.clone() />
                                                    })}
                                                    <div class="post-card__text">
                                                        <h2 class="post-card__heading">{next.title}</h2>
                                                        <p class="post-card__meta">{next.created_at_pretty}</p>
                                                        <p class="post-card__excerpt">{next.excerpt}</p>
                                                    </div>
                                                </a>
                                            </div>
                                        }
                                    })}
                                </div>
                            </div>
                        </div>
                    </div>
                }
            })}
    }
}

// Keep the `post::Post` reference alive so callers still compile
#[allow(dead_code)]
fn _post_module_alive(_: post::Post) {}
