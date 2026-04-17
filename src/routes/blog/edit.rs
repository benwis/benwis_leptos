use crate::functions::post::{UpdatePost, get_post};
#[cfg(not(feature = "ssr"))]
use crate::js;
use crate::models::post::Post;
use crate::providers::AuthContext;
use cfg_if::cfg_if;
use leptos::context::use_context;
use leptos::either::EitherOf4;
use leptos::prelude::*;
use leptos::server::{Resource, ServerAction};
use leptos::{IntoView, component, island, view};
use leptos_meta::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn EditPost() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let params = use_params_map();
    let post = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        get_post,
    );

    view! {
        <Transition fallback=|| ()>
            {move || Suspend::new(async move {
                match auth_context.user.await {
                    Ok(Some(_)) => Some(view! {
                        <Transition fallback=|| view! { <p>"Loading..."</p> }>
                            {move || Suspend::new(async move {
                                match post.await {
                                    Ok(Ok(Some(post))) => EitherOf4::A(
                                        view! { <EditPostForm post=post /> },
                                    ),
                                    Ok(Ok(None)) => EitherOf4::B(view! { <p>"Post Not Found"</p> }),
                                    Ok(Err(_)) => EitherOf4::C(view! { <p>"Server Error"</p> }),
                                    Err(_) => EitherOf4::D(view! { <p>"Server Fn Error"</p> }),
                                }
                            })}
                        </Transition>
                    }),
                    _ => None,
                }
            })}
        </Transition>
    }
}

#[island]
pub fn EditPostForm(post: Post) -> impl IntoView {
    let update_post = ServerAction::<UpdatePost>::new();
    let initial_html = post.content.clone();
    let initial_toc = post.toc.clone().unwrap_or_default();
    let content = RwSignal::new(initial_html);
    let toc = RwSignal::new(initial_toc);
    let show_post_metadata = RwSignal::new(false);

    let tags_value = serde_json::to_string(&post.tags).unwrap_or_else(|_| "[]".to_string());
    let post_id = post.id as i64;
    let raw_content = post.raw_content.clone();

    view! {
        <Meta property="og:title" content="Edit Post" />
        <Title text="Edit Post" />
        <Meta name="description" content="Edit a Post" />
        <Meta property="og:description" content="Edit a Post" />

        <ActionForm action=update_post>
            <div id="page">
                <div id="page__header">
                    <h1 id="page__heading">"Edit Post"</h1>
                </div>
                <section class="form__section">
                    <div
                        class="form__toggle"
                        on:click=move |_| show_post_metadata.update(|b| *b = !*b)
                    >
                        <span>{move || if show_post_metadata.get() { "−" } else { "+" }}</span>
                        <h2 class="form__section-heading">"Post Metadata"</h2>
                    </div>
                    {move || show_post_metadata.get().then(|| view! {
                        <div class="form__fields">
                            <input type="hidden" name="id" id="id" value=post_id />
                            <div class="form__field">
                                <label for="title">"Post Title"</label>
                                <input type="text" id="title" name="title" value=post.title.clone() />
                            </div>
                            <div class="form__field">
                                <label for="slug">"Post Slug"</label>
                                <input type="text" id="slug" name="slug" value=post.slug.clone() />
                            </div>
                            <div class="form__field">
                                <label for="created_at_pretty">"Post Date"</label>
                                <input
                                    type="text"
                                    id="created_at_pretty"
                                    name="created_at_pretty"
                                    value=post.created_at_pretty.clone()
                                />
                            </div>
                            <div class="form__field">
                                <label for="excerpt">"Excerpt"</label>
                                <textarea id="excerpt" name="excerpt" rows="5">
                                    {post.excerpt.clone().unwrap_or_default()}
                                </textarea>
                            </div>
                            <div class="form__field">
                                <label for="hero">"Hero"</label>
                                <input type="text" id="hero" name="hero" value=post.hero.clone().unwrap_or_default() />
                            </div>
                            <div class="form__field">
                                <label for="hero_alt">"Hero Alt"</label>
                                <input type="text" id="hero_alt" name="hero_alt" value=post.hero_alt.clone().unwrap_or_default() />
                            </div>
                            <div class="form__field">
                                <label for="hero_caption">"Hero Caption"</label>
                                <input type="text" id="hero_caption" name="hero_caption" value=post.hero_caption.clone().unwrap_or_default() />
                            </div>
                            <div class="form__field">
                                <label for="tags">"Tags (JSON array)"</label>
                                <input type="text" id="tags" name="tags" value=tags_value.clone() />
                            </div>
                            <div class="form__field">
                                <label for="preview">"Preview"</label>
                                <select id="preview" name="preview">
                                    <option value="false" selected=!post.preview>"False"</option>
                                    <option value="true" selected=post.preview>"True"</option>
                                </select>
                            </div>
                            <div class="form__field">
                                <label for="published">"Published"</label>
                                <select id="published" name="published">
                                    <option value="false" selected=!post.published>"False"</option>
                                    <option value="true" selected=post.published>"True"</option>
                                </select>
                            </div>
                        </div>
                    })}
                </section>
                <section class="form__actions">
                    <button type="submit" class="button">"Update Post"</button>
                </section>
                <div class="form__editor">
                    <input type="hidden" name="toc" id="toc" prop:value=move || toc.get() />
                    <label for="raw_content" class="form__hidden-label">"Content"</label>
                    <textarea
                        id="raw_content"
                        name="raw_content"
                        rows="50"
                        class="form__raw-content"
                        on:input=move |ev| {
                            cfg_if! { if #[cfg(not(feature = "ssr"))] {
                                let new_value = event_target_value(&ev);
                                match js::process_markdown_to_html_with_frontmatter(new_value) {
                                    Ok(o) => {
                                        content.set(o.content);
                                        toc.set(o.toc.unwrap_or_default());
                                    }
                                    Err(e) => leptos::logging::log!("{}", e.to_string()),
                                }
                            } else { let _ = &ev; }}
                        }
                    >
                        {raw_content}
                    </textarea>
                    <input
                        type="hidden"
                        name="content"
                        id="content"
                        prop:value=move || content.get()
                    />
                </div>
                <section class="form__preview">
                    <div
                        class="form__preview-body"
                        inner_html=move || content.get()
                    ></div>
                </section>
            </div>
        </ActionForm>
    }
}
