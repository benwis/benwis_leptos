use crate::functions::post::{get_post, UpdatePost};
use crate::models::post;
use crate::routes::blog::PostParams;
use leptos::*;
use leptos_router::*;
#[component]
pub fn EditPost(cx: Scope) -> impl IntoView {
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
    view! { cx,
        <Transition fallback=move || {
            view! { cx, <p>"Loading..."</p> }
        }>
            {
                let post = move || match post.read(cx) {
                    Some(Ok(Ok(Some(post)))) => {
                        view! { cx,
                            <main>
                                <EditPostForm post={post}/>
                            </main>
                        }
                            .into_any()
                    }
                    Some(Ok(Ok(None))) => {
                        view! { cx, <p>"Post Not Found"</p> }
                            .into_any()
                    }
                    Some(Ok(Err(_))) => {
                        view! { cx, <p>"Server Error"</p> }
                            .into_any()
                    }
                    Some(Err(_)) => {
                        view! { cx, <p>"Server Fn Error"</p> }
                            .into_any()
                    }
                    None => {
                        view! { cx, <h1>"Loading..."</h1> }
                            .into_any()
                    }
                };
                view! { cx, <main>{post}</main> }
            }
        </Transition>
    }
}

#[component]
pub fn EditPostForm(cx: Scope, post: post::Post) -> impl IntoView {

    let update_post = create_server_action::<UpdatePost>(cx);
    view! { cx,
        <ActionForm action=update_post class="text-black dark:text-white">
            <p>
                <label>"Post Title:"</label>
                <input
                    type="text"
                    name="title"
                    class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                    value={post.title}
                />
            </p>
            <p>
                <label>"Post Slug:"</label>
                <input
                    type="text"
                    name="slug"
                    class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                    value={post.slug}
                />
            </p>
            <p>
                <label>"Hero:"</label>
                <input
                    type="text"
                    name="hero"
                    class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                    value={post.hero}
                />
            </p>
            <p>
                <label>"Published:"</label>
                <select
                    name="published"
                    class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                >
                    <option value="false" selected={post.published.to_string()}>
                        "False"
                    </option>
                    <option value="true" selected={post.published.to_string()}>
                        "True"
                    </option>
                </select>
            </p>
            <p>
                <label>"Preview:"</label>
                <select
                    name="preview"
                    class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                >
                    <option value="false" selected={post.preview.to_string()}>
                        "False"
                    </option>
                    <option value="true" selected={post.preview.to_string()}>
                        "True"
                    </option>
                </select>
            </p>
            <p>
                <label>"Excerpt:"</label>
                <textarea id="excerpt" rows={5} name="excerpt" class="w-full text-black border border-gray-500">
                    {post.excerpt}
                </textarea>
            </p>
            <p>
                <label for="content">"Content:"</label>
                <br/>
                <textarea id="content" rows={20} name="content" class="w-full text-black border border-gray-500">
                    {post.content}
                </textarea>
            </p>
            <p class="text-right flex w-full justify-between">
                <button
                    type="submit"
                    class="rounded bg-blue-500 py-2 px-4 text-white hover:bg-blue-600 focus:bg-blue-400 disabled:bg-blue-300"
                >
                    "Edit Post"
                </button>
            </p>
        </ActionForm>
    }
}
