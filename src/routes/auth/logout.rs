use crate::functions;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::*;

#[component]
pub fn Logout(/*action: Action<functions::auth::Logout, Result<(), ServerFnError>>*/
) -> impl IntoView {
    //_ = &action.dispatch(functions::auth::Logout {});

    // TODO
    view! {  <Meta property="og:title" content="Logout"/>
    <Title text="Logout"/>
    <Meta name="description" content="Logout"/>
    <Meta property="og:description" content="Logout"/>
    <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
    }
    // TODO
    //    <Redirect path="/"/> */}
}
