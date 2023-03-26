use leptos::*;
use leptos_router::*;
use crate::functions::post::AddPost;

#[component]
pub fn AddPost(cx: Scope) -> impl IntoView{
    let add_post = create_server_action::<AddPost>(cx);
    
    view! { cx,
        <div class="flex min-h-full flex-col justify-center">
            <div class="mx-auto w-full max-w-md px-8">
                <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
                    "Add Post"
                </h1>
                <ActionForm action=add_post class="text-black dark:text-white">
                    <p>
                        <label>"Post Title:"</label>
                        <input
                            type="text"
                            name="title"
                            class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                        />
                    </p>
                    <p>
                        <label>"Post Slug:"</label>
                        <input
                            type="text"
                            name="slug"
                            class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                        />
                    </p>
                    <p>
                        <label>"Hero:"</label>
                        <input
                            type="text"
                            name="hero"
                            class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                        />
                    </p>
                    <p>
                        <label>"Published:"</label>
                        <select
                            name="published"
                            class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                        >
                            <option value="false">"False"</option>
                            <option value="true">"True"</option>
                        </select>
                    </p>
                    <p>
                        <label>"Preview:"</label>
                        <select
                            name="preview"
                            class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                        >
                            <option value="false">"False"</option>
                            <option value="true">"True"</option>
                        </select>
                    </p>
                    <p>
                        <label>"Excerpt:"</label>
                        <textarea
                            id="excerpt"
                            rows={5}
                            name="excerpt"
                            class="w-full text-black"
                        ></textarea>
                    </p>
                    <p>
                        <label for="content">"Content:"</label>
                        <br/>
                        <textarea
                            id="content"
                            rows={20}
                            class="w-full text-black"
                            name="content"
                        ></textarea>
                    </p>
                    <p class="text-right flex w-full justify-between">
                        <button
                            type="submit"
                            class="rounded bg-blue-500 py-2 px-4 text-white hover:bg-blue-600 focus:bg-blue-400 disabled:bg-blue-300"
                        >
                            "Add Post"
                        </button>
                    </p>
                </ActionForm>
            </div>
        </div>
    }
}