// Taken from https://github.com/tokio-rs/axum/blob/main/examples/parse-body-based-on-content-type/src/main.rs
// Modified to allow empty body instead of form content

use axum::{
    extract::{FromRequest, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestExt,
};

pub(crate) struct JsonOrEmpty<T>(pub(crate) T);

impl<S, T> FromRequest<S> for JsonOrEmpty<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    T: 'static + Default,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        match content_type {
            Some(content_type) => {
                if content_type.starts_with("application/json") {
                    let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                    return Ok(Self(payload));
                }
            }
            None => return Ok(Self(T::default())),
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}
