use crate::functions::post::get_post;
use crate::models::post;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
    pub slug: String,
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post = create_blocking_resource(
        move || params().map(|params| params.slug).ok().unwrap(),
        move |slug| get_post(slug),
    );

    view! {
        <Transition fallback=move || {
            view! {  <p>"Loading..."</p> }
        }>
            { move || post.get().map(|p|{ match p {
                Ok(Ok(Some(post))) => {
                    view! {  <PostContent post={post}/> }
                        .into_view()
                }
                Ok(Ok(None)) => {
                    view! {  <p>"Post Not Found"</p> }
                        .into_view()
                }
                Ok(Err(_)) => {
                    view! {  <p>"Server Error"</p> }
                        .into_view()
                }
                Err(_) => {
                    view! {  <p>"Server Fn Error"</p> }
                        .into_view()
                }
            }})
            }
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
                <Meta property="og:title" content={post.title.clone()}/>
                <Meta property="og:description" content={post.excerpt.clone().unwrap_or_default()}/>
                <Meta property="og:site_name" content="benw.is"/>
                <Meta property="og:locale" content="en-us"/>
                <Meta property="og:type" content="article"/>
                <Meta name="og:image" content="https://benwis.imgix.net/ben_catcarbon.png"/>
                <Meta property="og:url" content={format!("https://benw.is/posts/{}", post.slug.clone())}/>
                <Meta name="twitter:title" content={post.title.clone()}/>
                <Meta name="twitter:site" content="@iambenwis"/>
                <Title text={post.title.clone()} />
                <Meta name="twitter:card" content="summary"/>
                <Meta name="twitter:image" content="https://benwis.imgix.net/ben_catcarbon.png"/>
                <Meta name="twitter:description" content={post.excerpt.clone().unwrap_or_default()}/>
                <Meta name="description" content={post.excerpt.clone().unwrap_or_default()}/>
            </div>
            {(post.preview || post.published)
                .then(|| {
                    view! {
                        <h1 class="mb-4 text-3xl text-black dark:text-white md:text-5xl">{post.title.clone()}</h1>
                        <div class="dark:text-white text-black mb-2">{post.created_at.to_string()}</div>
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
                            inner_html={post.content}
                        ></section>
                    }
                })}
        </section>
    }
}
