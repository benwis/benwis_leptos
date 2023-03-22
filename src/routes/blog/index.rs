use crate::functions::post::{get_posts, AddPost, DeletePost};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
    let add_post = create_server_multi_action::<AddPost>(cx);
    let delete_post = create_server_action::<DeletePost>(cx);
    let submissions = add_post.submissions();

    // list of posts is loaded from the server in reaction to changes
    let posts = create_resource(
        cx,
        move || (add_post.version().get(), delete_post.version().get()),
        move |_| get_posts(cx),
    );

    view! {
        cx,
        <div>
            <MultiActionForm action=add_post>
                <label>
                    "Add a Post"
                    <input type="text" name="title"/>
                    <input type="text" name="slug"/>
                    <textarea type="text" name="excerpt"/>
                    <textarea type="text" name="content"/>

                </label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
            <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                {move || {
                    let existing_posts = {
                        move || {
                            posts.read(cx)
                                .map(move |posts| match posts {
                                    Err(e) => {
                                        vec![view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_any()]
                                    }
                                    Ok(posts) => {
                                        if posts.is_empty() {
                                            vec![view! { cx, <p>"No posts were found."</p> }.into_any()]
                                        } else {
                                            posts
                                                .into_iter()
                                                .map(move |post| {
                                                    view! {
                                                        cx,
                                                        <section>
                                                        <A href={post.slug} class="no-underline hover:underline hover:decoration-yellow-400">
                                                            <li class="mb-8 text-lg">
                                                                <div class="inline-flex justify-between w-full">
                                                                    <h4 class="text-lg font-medium md:text-xl text-black dark:text-white">{post.title}</h4>
                                                                    <p class=" text-left text-gray-500 dark:text-gray-400 md:mb-0 md:text-right">{post.created_at}</p>
                                                                </div>
                                                                <p class="text-gray-500">{post.excerpt}</p>

                                                            </li>
                                                        </A>
                                                        <ActionForm action=delete_post>
                                                        <input type="hidden" name="id" value={post.id}/>
                                                        <input type="submit" value="X"/>
                                                        </ActionForm>
                                                        </section>
                                                    }.into_any()
                                                        // <li>
                                                        // <section>
                                                        // <h2> {post.title}</h2>
                                                        // <h4>{post.created_at}</h4>
                                                        // <h4>{post.user.unwrap_or_default().username}</h4>
                                                        // <div inner_html={post.content}></div>
                                                        // </section>
                                                        //     <ActionForm action=delete_post>
                                                        //         <input type="hidden" name="id" value={post.id}/>
                                                        //         <input type="submit" value="X"/>
                                                        //     </ActionForm>
                                                        // </li>
                                                    //}

                                                })
                                                .collect::<Vec<_>>()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };

                    let pending_posts = move || {
                        submissions
                        .get()
                        .into_iter()
                        .filter(|submission| submission.pending().get())
                        .map(|submission| {
                            view! {
                                cx,
                                <li class="pending">{move || submission.input.get().map(|data| data.title) }</li>
                            }
                        })
                        .collect::<Vec<_>>()
                    };

                    view! {
                        cx,
                        <ul>
                            {pending_posts}
                            {existing_posts}
                        </ul>
                    }
                }
            }
            </Transition>
        </div>
    }
}
