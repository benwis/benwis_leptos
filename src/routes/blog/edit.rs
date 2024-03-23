use crate::functions::post::{get_post, UpdatePost};
use crate::models::post;
use crate::routes::blog::PostParams;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post = create_resource(
        move || params.get().map(|params| params.slug).ok().unwrap().unwrap(),
        // any of the following would work (they're identical)
        // move |id| async move { get_contact(id).await }
        // move |id| get_contact(id),
        // get_contact
        move |slug| get_post(slug),
    );
    view! {
        <Transition fallback=move || {
            view! {  <p>"Loading..."</p> }
        }>
            {
                 move || post.get().map(|p| match p {
                    Ok(Ok(Some(post))) => {
                        view! {
                                <EditPostForm post={post}/>
                        }
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
                })
            }
        </Transition>
    }
}

#[component]
pub fn EditPostForm(post: post::Post) -> impl IntoView {
    let update_post = create_server_action::<UpdatePost>();
    view! {
        <Meta property="og:title" content="Edit Post"/>
        <Title text="Edit Post"/>
        <Meta name="description" content="Edit a Post"/>
        <Meta property="og:description" content="Edit a Post"/>
        <ActionForm action=update_post class="text-black dark:text-white w-full">
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
            <label>"Created At:"</label>
            <input
                type="text"
                name="created_at_pretty"
                placeholder="1970-01-01 00:00:00"
                class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                value={post.created_at_pretty}
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
                <textarea
                    id="excerpt"
                    rows={5}
                    name="excerpt"
                    class="w-full text-black border border-gray-500"
                >
                    {post.excerpt}
                </textarea>
            </p>
            <p>
                <label for="content">"Content:"</label>
                <br/>
                <textarea
                    id="content"
                    rows={20}
                    name="content"
                    class="w-full text-black border border-gray-500"
                >
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
