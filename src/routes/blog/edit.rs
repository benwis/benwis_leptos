use crate::functions::post::get_post;
use crate::models::post;
use crate::routes::blog::PostParams;
use leptos::*;
use leptos_router::*;
#[component]
pub fn PostEdit(cx: Scope) -> impl IntoView {
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
                    <PostEditForm post={post}/>
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
pub fn PostEditForm(cx: Scope, post: post::Post) -> impl IntoView {
    view! {cx,
        <h1>{post.title}</h1>
        <p inner_html={post.content}/>
    }
}
