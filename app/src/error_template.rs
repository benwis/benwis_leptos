use leptos::{IntoView, component, prelude::*, view};

#[component]
pub fn ErrorTemplate(
    #[prop(optional)] status_code: Option<u16>,
    #[prop(optional)] message: Option<String>,
) -> impl IntoView {
    let code = status_code.unwrap_or(404);
    let msg = message.unwrap_or_else(|| match code {
        404 => "Page not found".to_string(),
        401 => "Authentication required".to_string(),
        500 => "Internal server error".to_string(),
        _ => "Something went wrong".to_string(),
    });

    view! {
        <div class="error-template" style="text-align: center; padding: 4rem 1rem;">
            <h1 style="font-size: 3rem; margin-bottom: 0.5rem;">{code}</h1>
            <p style="font-size: 1.25rem; color: var(--text-secondary, #666);">{msg}</p>
            <a href="/" style="display: inline-block; margin-top: 2rem;">"Go home"</a>
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = leptos::prelude::use_context::<leptos_axum::ResponseOptions>() {
            resp.set_status(http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <ErrorTemplate status_code=404 />
    }
}
