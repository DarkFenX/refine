use axum::{
    body::{Body, to_bytes},
    extract::{Request, State},
    http,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::LengthLimitError;

use crate::{
    err::ApiError,
    logging::{LogBodies, RX_PREFIX, TX_PREFIX},
};

#[derive(Copy, Clone)]
pub(crate) struct BodyLimit {
    pub(crate) max_request_body_size: u64,
    pub(crate) log_bodies: LogBodies,
}

// Built-in body limit does not allow customizing behavior; this custom middle-ware exists because
// of that
pub(crate) async fn limit_request_body_size(State(limit): State<BodyLimit>, req: Request, next: Next) -> Response {
    match get_content_len(&req) {
        Some(content_len) if content_len > limit.max_request_body_size => reject(Some(content_len), limit).await,
        Some(..) => next.run(req).await,
        // Sometimes size might be not declared (e.g. chunked requests), separate handling for this
        // case
        None => {
            let (parts, body) = req.into_parts();
            match to_bytes(body, limit.max_request_body_size as usize).await {
                Ok(bytes) => next.run(Request::from_parts(parts, Body::from(bytes))).await,
                // Errors might happen because the limit has been broken, and due to other reasons;
                // figure out the cause and respond accordingly
                Err(error) => match is_over_limit(&error) {
                    true => reject(None, limit).await,
                    false => ApiError::RequestRead(error).into_response(),
                },
            }
        }
    }
}

fn get_content_len(request: &Request) -> Option<u64> {
    request
        .headers()
        .get(http::header::CONTENT_LENGTH)?
        .to_str()
        .ok()?
        .parse()
        .ok()
}

async fn reject(size: Option<u64>, limit: BodyLimit) -> Response {
    let msg = format!(
        "received length {} is bigger than limit {}",
        match size {
            Some(size) => size.to_string(),
            None => "<unknown>".to_string(),
        },
        limit.max_request_body_size
    );
    match limit.log_bodies {
        // Replicate body logging middleware format; have to do it here because rejected requests
        // never reach it
        LogBodies::Enabled => {
            tracing::info!("{RX_PREFIX} body: <{}>", msg);
            let response = ApiError::RequestTooLarge(msg).into_response();
            let (parts, body) = response.into_parts();
            let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
            tracing::info!("{TX_PREFIX} body: {}", String::from_utf8_lossy(&bytes));
            Response::from_parts(parts, Body::from(bytes))
        }
        LogBodies::Disabled => ApiError::RequestTooLarge(msg).into_response(),
    }
}

fn is_over_limit(error: &axum::Error) -> bool {
    // Since axum errors do not expose anything like serde's is_io(), have to figure out error type
    // via source downcasting
    std::error::Error::source(error).is_some_and(|source| source.is::<LengthLimitError>())
}
