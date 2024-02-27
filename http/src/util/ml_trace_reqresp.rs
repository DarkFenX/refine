// Taken from https://github.com/tokio-rs/axum/blob/main/examples/print-request-response/src/main.rs
// Might need updates from time to time, due to axum changes

use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;

pub(crate) async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print(">>>", "rx", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("<<<", "tx", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(arrows: &str, direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, format!("{arrows} failed to read body: {err}")));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        if body.is_empty() {
            tracing::info!("{arrows} {direction} body is empty");
        } else {
            tracing::info!("{arrows} {direction} body: {body}");
        }
    }
    Ok(bytes)
}
