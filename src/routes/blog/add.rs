use crate::functions::post::AddPost;
use leptos::prelude::*;
use leptos::server::ServerAction;
use leptos::{component, view, IntoView};
use leptos_meta::*;

#[component]
pub fn AddPost() -> impl IntoView {
    let add_post = ServerAction::<AddPost>::new();

    view! {
        <Meta property="og:title" content="Add Post" />
        <Title text="Add Post" />
        <Meta name="description" content="Add a post" />
        <Meta property="og:description" content="Add a post" />
        <div class="flex min-h-full w-full flex-col justify-center">
            <div class="mx-auto w-full px-8">
                <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
                    "Add Post"
                </h1>
                // TODO // class="w-full text-black dark:text-white">
                <ActionForm action=add_post>
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
                        <label>"Created At:"</label>
                        <input
                            type="text"
                            name="created_at_pretty"
                            placeholder="1970-01-01 00:00:00"
                            class="w-full rounded border border-gray-500 px-2 py-1 text-lg text-black bg-white"
                        />
                    </p>
                    // TODO fixme probably related to <option> in macro

                    <p>
                        <label>"Excerpt:"</label>
                        <textarea
                            id="excerpt"
                            // {5}
                            rows="5"
                            name="excerpt"
                            class="w-full text-black border border-gray-500"
                        ></textarea>
                    </p>
                    <p>
                        <label for="content">"Content:"</label>
                        <br />
                        <textarea
                            id="content"
                            // TODO primitives as attribute values {20}
                            rows="20"
                            class="w-full text-black border border-gray-500"
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
