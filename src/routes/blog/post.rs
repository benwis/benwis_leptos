use crate::functions::post::get_post;
use crate::models::post;
use crate::providers::AuthContext;
use leptos::reactive_graph::computed::Memo;
use leptos::reactive_graph::owner::use_context;
use leptos::server::Resource;
use leptos::tachys::either::{Either, EitherOf4};
use leptos::{component, IntoView};
use leptos::{prelude::*, view};
use leptos_meta::*;
use routing::RouteData;
/*
#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
    pub slug: String,
}*/

pub fn Post(route_data: RouteData) -> impl IntoView {
    let params = Memo::from(route_data.params);
    // TODO typed param decoding
    // TODO blocking resources
    let post = Resource::new_serde(move || async move {
        let slug = params
            .with(|p| p.get("slug").map(ToOwned::to_owned))
            .unwrap_or_default();
        get_post(slug).await
    });

    move || {
        async move {
            match post.await {
                Ok(Ok(Some(post))) => EitherOf4::A(view! {  <PostContent post={post}/> }),
                Ok(Ok(None)) => EitherOf4::B(view! {  <p>"Post Not Found"</p> }),
                Ok(Err(_)) => EitherOf4::C(view! {  <p>"Server Error"</p> }),
                Err(_) => EitherOf4::D(view! {  <p>"Server Fn Error"</p> }),
            }
        }
        .suspend()
        .transition()
        .with_fallback(view! { <p>"Loading..."</p> })
        .track()
    }
}

#[component]
pub fn PostContent(post: post::Post) -> impl IntoView {
    //let auth_context = use_context::<AuthContext>().expect("Failed to get Auth Context");

    view! {
        <section class="px-4 w-full">
            <div class="flex justify-between w-full">
                <a href="/posts" class="dark:text-white">
                    "Back to Posts"
                </a>
                <Meta property="og:title" content={post.title.clone()}/>
                <Meta property="og:description" content={post.excerpt.clone().unwrap_or_default()}/>
                <Meta property="og:site_name" content="benw.is"/>
                <Meta property="og:locale" content="en-us"/>
                <Meta property="og:type" content="article"/>
                <Meta property="og:url" content={post.excerpt.clone().unwrap_or_default()}/>
                <Meta name="twitter:title" content={post.title.clone()}/>
                <Meta name="twitter:site" content="@iambenwis"/>
                <Title text={post.title.clone()} />
                <Meta name="twitter:card" content="summary"/>
                <Meta name="twitter:image" content="https://benwis.imgix.net/ben_catcarbon.jpeg"/>
                <Meta name="twitter:description" content={post.excerpt.clone().unwrap_or_default()}/>
                <Meta name="description" content={post.excerpt.clone().unwrap_or_default()}/>
                // TODO
                /*<Transition fallback=|| ()>
                    {move || {
                        match auth_context.user.read() {
                            Some(Ok(user)) => {
                                view! {
                                    <Show when=move || user.is_some() fallback=|| ()>
                                        <A class="dark:text-white no-underline" href="edit">
                                            "Edit"
                                        </A>
                                    </Show>
                                }
                                    .into_view()
                            }
                            Some(Err(_)) => ().into_view(),
                            None => ().into_view(),
                        }
                    }}
                </Transition>*/
            </div>
            {(post.preview || post.published)
                .then(|| {
                    view! {
                        <h1 class="mb-4 text-3xl text-black dark:text-white md:text-5xl">{post.title.clone()}</h1>
                        <div class="dark:text-white text-black mb-2">{post.created_at_pretty}</div>
                        <div class="-mx-4 my-2 flex h-1 w-[100vw] bg-gradient-to-r from-yellow-400 via-rose-400 to-cyan-500 sm:mx-0 sm:w-full"></div>
                        <section class="dark:bg-gray-800 p-4 mt-4 table-of-contents-parent">
                            <h2 class="text-xl text-black dark:text-white md:text-2xl">"Contents"</h2>
                            <div
                                class="text-black prose lg:prose-xl dark:prose-invert dark:text-white text-base md: w-full"
                                inner_html={post.toc}
                            ></div>
                        </section>
                        <section
                            class="text-black mx-auto prose lg:prose-xl dark:prose-invert dark:text-white text-base mt-8"
                            inner_html={post.html}
                        ></section>
                    }
                })}
        </section>
    }
}
