use crate::error_template::*;
use crate::layouts::Default;
use crate::providers::{provide_auth, provide_color_scheme, AuthContext};
use crate::routes::auth::{Join, Login, Logout};
use crate::routes::blog::*;
use crate::routes::{About, Index, Nedry, Portfolio};
use leptos::reactive_graph::owner::use_context;
use leptos::{component, IntoView};
use leptos::{prelude::*, view};
use leptos_meta::*;
use routing::location::{BrowserUrl, RequestUrl};
use routing::{NestedRoute, ParamSegment, Router, Routes, StaticSegment};

#[component]
pub fn BenwisApp() -> impl IntoView {
    // Create Actions for the Auth methods and provide the current user
    provide_auth();
    //    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContezt");
    _ = provide_color_scheme();

    provide_meta_context();

    // TODO better API and component version of this
    Router::<_, BrowserUrl, _, _>::new(
        Routes::new(
            NestedRoute::new(StaticSegment(""), |route_data| {
                view! {
                    <Default>
                        {route_data.outlet}
                    </Default>
                }
            })
            .child((
                NestedRoute::new(StaticSegment(""), |_| Index()),
                NestedRoute::new(StaticSegment("signup"), |_| Join()),
                NestedRoute::new(StaticSegment("about"), |_| About()),
                NestedRoute::new(StaticSegment("portfolio"), |_| Portfolio()),
                NestedRoute::new(StaticSegment("posts"), |_| Blog()),
                NestedRoute::new((StaticSegment("posts"), StaticSegment("about")), |_| {
                    AddPost()
                }),
                NestedRoute::new((StaticSegment("posts"), ParamSegment("slug")), Post),
                NestedRoute::new(
                    (
                        StaticSegment("posts"),
                        ParamSegment("slug"),
                        StaticSegment("edit"),
                    ),
                    |_| EditPost(),
                ),
                NestedRoute::new(StaticSegment("login"), |_| Login()),
                NestedRoute::new(StaticSegment("logout"), |_| Logout()),
                NestedRoute::new(StaticSegment("nedry"), |_| Nedry()),
            )),
        ),
        || "Not found!",
    )
}
