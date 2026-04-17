use crate::functions;
use leptos::{IntoView, component, server::ServerAction, view};
use leptos_meta::*;
use leptos_router::components::Redirect;

#[component]
pub fn Logout(action: ServerAction<functions::auth::Logout>) -> impl IntoView {
    _ = &action.dispatch(functions::auth::Logout {});

    view! {
        <Meta property="og:title" content="Logout" />
        <Title text="Logout" />
        <Meta name="description" content="Logout" />
        <Meta property="og:description" content="Logout" />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg" />
        <Redirect path="/" />
    }
}
