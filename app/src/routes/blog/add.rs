use crate::functions::post::AddPost;
#[cfg(not(feature = "ssr"))]
use crate::js;
use crate::providers::AuthContext;
use cfg_if::cfg_if;
use leptos::context::use_context;
use leptos::prelude::*;
use leptos::server::ServerAction;
use leptos::{IntoView, component, island, view};
use leptos_meta::*;

#[component]
pub fn AddPost() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    view! {
        <Meta property="og:title" content="Add Post" />
        <Title text="Add Post" />
        <Meta name="description" content="Add a post" />
        <Meta property="og:description" content="Add a post" />

        <Transition fallback=|| ()>
            {move || Suspend::new(async move {
                match auth_context.user.await {
                    Ok(Some(_)) => Some(view! { <AddPostForm /> }),
                    _ => None,
                }
            })}
        </Transition>
    }
}

#[island]
pub fn AddPostForm() -> impl IntoView {
    let add_post = ServerAction::<AddPost>::new();
    let content = RwSignal::new(String::new());
    let toc = RwSignal::new(String::new());
    let show_post_metadata = RwSignal::new(false);

    view! {
        <ActionForm action=add_post>
            <div id="editor">
                <div id="page__header">
                    <h1 id="page__heading">"Add Post"</h1>
                </div>
                <div
                    on:click=move |_| show_post_metadata.update(|b| *b = !*b)
                    style="cursor:pointer; padding: 0.5rem 0; user-select:none;"
                >
                    <span>{move || if show_post_metadata.get() { "−" } else { "+" }}</span>
                    " Post Metadata"
                </div>
                {move || show_post_metadata.get().then(|| view! {
                    <div id="editor__meta">
                        <div class="editor__meta-row">
                            <label for="title">"Post Title"</label>
                            <input type="text" id="title" name="title" />
                        </div>
                        <div class="editor__meta-row">
                            <label for="slug">"Post Slug"</label>
                            <input type="text" id="slug" name="slug" />
                        </div>
                        <div class="editor__meta-row">
                            <label for="created_at_pretty">"Post Date"</label>
                            <input
                                type="text"
                                id="created_at_pretty"
                                name="created_at_pretty"
                                placeholder="1970-01-01 00:00:00"
                            />
                        </div>
                        <div class="editor__meta-row">
                            <label for="excerpt">"Excerpt"</label>
                            <textarea id="excerpt" name="excerpt" rows="5"></textarea>
                        </div>
                        <div class="editor__meta-row">
                            <label for="hero">"Hero"</label>
                            <input type="text" id="hero" name="hero" />
                        </div>
                        <div class="editor__meta-row">
                            <label for="hero_alt">"Hero Alt"</label>
                            <input type="text" id="hero_alt" name="hero_alt" />
                        </div>
                        <div class="editor__meta-row">
                            <label for="hero_caption">"Hero Caption"</label>
                            <input type="text" id="hero_caption" name="hero_caption" />
                        </div>
                        <div class="editor__meta-row">
                            <label for="tags">"Tags (comma separated)"</label>
                            <input type="text" id="tags" name="tags" />
                        </div>
                        <div class="editor__meta-row">
                            <label for="preview">"Preview"</label>
                            <select id="preview" name="preview">
                                <option selected value="false">"False"</option>
                                <option value="true">"True"</option>
                            </select>
                        </div>
                        <div class="editor__meta-row">
                            <label for="published">"Published"</label>
                            <select id="published" name="published">
                                <option selected value="false">"False"</option>
                                <option value="true">"True"</option>
                            </select>
                        </div>
                    </div>
                })}
                <div style="padding: 0.5rem 2.5rem;">
                    <button type="submit" class="button">"Add Post"</button>
                </div>
                <div id="editor__body">
                    <div id="editor__body-raw">
                        <input type="hidden" name="toc" id="toc" prop:value=move || toc.get() />
                        <textarea
                            id="raw_content"
                            name="raw_content"
                            rows="50"
                            style="border:none; font-family:'Berkeley Mono',monospace,system-ui;"
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
                        ></textarea>
                        <input
                            type="hidden"
                            name="content"
                            id="content"
                            prop:value=move || content.get()
                        />
                    </div>
                    <div id="editor__body-formatted" class="content">
                        <div inner_html=move || content.get()></div>
                    </div>
                </div>
            </div>
        </ActionForm>
    }
}
