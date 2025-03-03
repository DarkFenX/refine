// Taken from https://github.com/tokio-rs/axum/blob/main/examples/print-request-response/src/main.rs
// Might need updates from time to time, due to axum changes

use axum::{
    body::{Body, Bytes, to_bytes},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

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

async fn buffer_and_print(arrows: &str, direction: &str, body: Body) -> Result<Bytes, (StatusCode, String)> {
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
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
