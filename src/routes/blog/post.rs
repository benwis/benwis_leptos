use crate::functions::post::get_post;
use crate::models::post;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
    pub slug: String,
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params::<PostParams>(cx);
    let post = create_resource(
        cx,
        move || params().map(|params| params.slug).ok().unwrap(),
        // any of the following would work (they're identical)
        // move |id| async move { get_contact(id).await }
        // move |id| get_contact(id),
        // get_contact
        move |slug| get_post(cx, slug),
    );

    view! {
            cx,
           <Transition fallback=move || view! {cx, <p>"Loading..."</p>}>
           {
            let post = move || match post.read(cx){
                Some(Ok(Ok(Some(post)))) => view!{cx,
                    <main>
                    <PostContent post={post}/>
                    </main>
                }.into_any(),
                Some(Ok(Ok(None))) => view!{cx,
                    <p>"Post Not Found"</p>
                }.into_any(),
                Some(Ok(Err(_))) => view!{cx,
                    <p>"Server Error"</p>
                }.into_any(),
                Some(Err(_)) => view!{cx,
                    <p>"Server Fn Error"</p>
                }.into_any(),
                None => view!{cx,
                    <h1>"Loading..."</h1>
                }.into_any()
            };

            view!{cx,
            <main>{post}</main>
        }
    }
           </Transition>
        }
}

#[component]
pub fn PostContent(cx: Scope, post: post::Post) -> impl IntoView {
    view! {cx,
        <main class="px-4 max-w-5xl">
        <div class="">

        <div class="flex justify-between w-full">
            <a href="/posts" class="dark:text-white">"Back to Posts"</a>

        //     {admin ? <div class="dark:text-white">
        //     <a class="dark:text-white no-underline" href="edit"> Edit</a>
        // </div> : null}

        </div>
        <h1 class="mb-4 text-3xl text-black dark:text-white md:text-5xl">
          {post.title}
        </h1>
        <div class="dark:text-white text-black mb-2">
            {post.created_at}
        </div>
        <div
        class="-mx-4 my-2 flex h-1 w-[100vw] bg-gradient-to-r from-yellow-400 via-rose-400 to-cyan-500 sm:mx-0 sm:w-full"
    />
        // Table of Contents
        <section class="dark:bg-gray-800 p-4 mt-4 table-of-contents-parent">
            <h2 class="text-xl text-black dark:text-white md:text-2xl">"Contents"</h2>
            <div class="text-black prose lg:prose-xl dark:prose-invert dark:text-white text-base md: w-full" inner_html={ post.toc } />
          </section>

        // Post content
        <div class="text-black prose lg:prose-xl dark:prose-invert dark:text-white text-base mt-8" inner_html={post.content} />
        </div>
      </main>

    }
}
