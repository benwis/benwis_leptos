use app::error_template::ErrorTemplate;
use errors::errors::BenwisAppError;
use axum::response::Response as AxumResponse;
use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::{view, Errors, LeptosOptions};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let mut res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        let headers = res.headers_mut();
        let uri_string = uri.to_string();
        if !(uri_string.contains(".js")
            || uri_string.contains(".wasm")
            || uri_string.contains(".css"))
        {
            headers.insert(
                http::header::CACHE_CONTROL,
                http::HeaderValue::from_str("public, max-age=31536000").unwrap(),
            );
        }
        res.into_response()
    } else {
        let mut errors = Errors::default();
        errors.insert_with_default_key(BenwisAppError::NotFound);
        let handler = leptos_axum::render_app_to_stream(
            options.to_owned(),
            move || view! { <ErrorTemplate outside_errors=errors.clone()/>},
        );
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
