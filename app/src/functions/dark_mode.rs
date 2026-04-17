use leptos::context::use_context;
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use leptos::{IntoView, component, server, view};

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(prefers_dark: bool) -> Result<bool, ServerFnError> {
    use axum::http::{HeaderMap, HeaderValue, header::SET_COOKIE};
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>().expect("to have leptos_axum::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;

    response.overwrite(response_parts);

    // In islands mode the form submits as a plain POST, so redirect back
    // to the referring page instead of returning a raw response body.
    if let Some(req) = use_context::<http::request::Parts>() {
        if let Some(referer) = req.headers.get(http::header::REFERER) {
            if let Ok(r) = referer.to_str() {
                leptos_axum::redirect(r);
            }
        }
    }

    Ok(prefers_dark)
}
