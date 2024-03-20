use crate::functions;
use leptos::{component, server::ServerAction, view, IntoView};
use leptos_meta::*;

#[component]
pub fn Logout(action: ServerAction<functions::auth::Logout>) -> impl IntoView {
    _ = &action.dispatch(functions::auth::Logout {});

    view! {  <Meta property="og:title" content="Logout"/>
    <Title text="Logout"/>
    <Meta name="description" content="Logout"/>
    <Meta property="og:description" content="Logout"/>
    <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
    }
    // TODO
    //    <Redirect path="/"/> */}
}
