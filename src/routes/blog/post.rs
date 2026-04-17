use crate::functions::post::get_post;
use crate::models::post;
use leptos::either::EitherOf4;
use leptos::server::Resource;
use leptos::{component, IntoView};
use leptos::{prelude::*, view};
use leptos_meta::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let post = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        get_post,
    );

    view! {
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            {move || {
                Suspend::new(async move {
                    match post.await {
                        Ok(Ok(Some(post))) => EitherOf4::A(view! { <PostContent post=post /> }),
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
pub fn PostContent(post: post::Post) -> impl IntoView {
    view! {
        <section class="px-4 w-full">
            <div class="flex justify-between w-full">
                <a href="/posts" class="dark:text-white">
                    "Back to Posts"
                </a>
                <Meta property="og:title" content=post.title.clone() />
                <Meta property="og:description" content=post.excerpt.clone().unwrap_or_default() />
                <Meta property="og:site_name" content="benw.is" />
                <Meta property="og:locale" content="en-us" />
                <Meta property="og:type" content="article" />
                <Meta property="og:url" content=post.excerpt.clone().unwrap_or_default() />
                <Meta name="twitter:title" content=post.title.clone() />
                <Meta name="twitter:site" content="@iambenwis" />
                <Title text=post.title.clone() />
                <Meta name="twitter:card" content="summary" />
                <Meta name="twitter:image" content="https://benwis.imgix.net/ben_catcarbon.jpeg" />
                <Meta name="twitter:description" content=post.excerpt.clone().unwrap_or_default() />
                <Meta name="description" content=post.excerpt.clone().unwrap_or_default() />
            // TODO
            </div>
            {(post.preview || post.published)
                .then(|| {
                    view! {
                        <h1 class="mb-4 text-3xl text-black dark:text-white md:text-5xl">
                            {post.title.clone()}
                        </h1>
                        <div class="dark:text-white text-black mb-2">{post.created_at_pretty}</div>
                        <div class="-mx-4 my-2 flex h-1 w-[100vw] bg-gradient-to-r from-yellow-400 via-rose-400 to-cyan-500 sm:mx-0 sm:w-full"></div>
                        <section class="dark:bg-gray-800 p-4 mt-4 table-of-contents-parent">
                            <h2 class="text-xl text-black dark:text-white md:text-2xl">
                                "Contents"
                            </h2>
                            <div
                                class="text-black prose lg:prose-xl dark:prose-invert dark:text-white text-base md: w-full"
                                inner_html=post.toc
                            ></div>
                        </section>
                        <section
                            class="text-black mx-auto prose lg:prose-xl dark:prose-invert dark:text-white text-base mt-8"
                            inner_html=post.html
                        ></section>
                    }
                })}
        </section>
    }
}
