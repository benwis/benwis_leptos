use crate::functions::post::get_post;
use crate::models::post;
use crate::providers::AuthContext;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
    pub slug: String,
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params::<PostParams>(cx);
    let post = create_deferred_resource(
        cx,
        move || params().map(|params| params.slug).ok().unwrap(),
        move |slug| get_post(cx, slug),
    );

    view! { cx,
        <Transition fallback=move || {
            view! { cx, <p>"Loading..."</p> }
        }>
            { move || post.read(cx).map(|p|{ match p {
                Ok(Ok(Some(post))) => {
                    view! { cx, <PostContent post={post}/> }
                        .into_view(cx)
                }
                Ok(Ok(None)) => {
                    view! { cx, <p>"Post Not Found"</p> }
                        .into_view(cx)
                }
                Ok(Err(_)) => {
                    view! { cx, <p>"Server Error"</p> }
                        .into_view(cx)
                }
                Err(_) => {
                    view! { cx, <p>"Server Fn Error"</p> }
                        .into_view(cx)
                }
            }})
            }
        </Transition>
    }
}

#[component]
pub fn PostContent(cx: Scope, post: post::Post) -> impl IntoView {
    let auth_context = use_context::<AuthContext>(cx).expect("Failed to get Auth Context");

    view! { cx,
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
                <Meta name="description" content={post.excerpt.clone().unwrap_or_default()}/>                <Transition fallback=|| ()>
                    {move || {
                        match auth_context.user.read(cx) {
                            Some(Ok(user)) => {
                                view! { cx,
                                    <Show when=move || user.is_some() fallback=|_| ()>
                                        <A class="dark:text-white no-underline" href="edit">
                                            "Edit"
                                        </A>
                                    </Show>
                                }
                                    .into_view(cx)
                            }
                            Some(Err(_)) => ().into_view(cx),
                            None => ().into_view(cx),
                        }
                    }}
                </Transition>
            </div>
            {(post.preview || post.published)
                .then(|| {
                    view! { cx,
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
