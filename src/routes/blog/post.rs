use crate::functions::post::{get_post, AddPost, DeletePost};
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
    slug: String,
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
            <p>"Hi"</p>
    //        <Transition fallback=move || view! {cx, <p>"Loading..."</p>}>
    //        {
    //         let post = move || match post.read(cx){
    //             Some(Ok(r)) => Ok(r),
    //             Some(Err(e)) => Err(e),
    //             None => "oading"
    //         };

    //         view!{cx,
    //         <p>"Hi"</p>
    //     }
    // }
    //        </Transition>
        }
}
