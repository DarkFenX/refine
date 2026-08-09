//! Initially code was taken from axum example:
//! <https://github.com/tokio-rs/axum/blob/main/examples/print-request-response/src/main.rs>

use axum::{
    body::{Body, Bytes, to_bytes},
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    err::ApiError,
    logging::{RX_PREFIX, TX_PREFIX},
};

pub(crate) async fn log_request_response(req: Request, next: Next) -> Result<impl IntoResponse, ApiError> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_log(RX_PREFIX, body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_log(TX_PREFIX, body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_log(prefix: &str, body: Body) -> Result<Bytes, ApiError> {
    // Body limit is applied by different middleware which is supposed to run before this
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        // Respond with request error, because only requests can fail, responses are already stored
        // in memory
        Err(error) => return Err(ApiError::RequestRead(error)),
    };
    // Before changing those logging levels, consider other places which rely on those:
    // - body limit middleware attempts to replicate the same logging;
    // - logging setup final decision to log bodies or not depends on knowing logging level in here.
    match std::str::from_utf8(&bytes) {
        Ok(body) => match body.is_empty() {
            true => tracing::info!("{prefix} body is empty"),
            false => tracing::info!("{prefix} body: {body}"),
        },
        Err(..) => tracing::info!("{prefix} body: <invalid UTF-8, {} bytes>", bytes.len()),
    }
    Ok(bytes)
}
