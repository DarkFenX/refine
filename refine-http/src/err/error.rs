use axum::{
    Json,
    extract::rejection::{JsonRejection, QueryRejection},
    response::IntoResponse,
};
use serde::Serialize;

use crate::err::ApiErrorIndexed;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ApiError {
    #[error("{}", .0.body_text())]
    Query(QueryRejection),
    #[error("{}", .0.body_text())]
    Json(JsonRejection),
    #[error("failed to read request body")]
    RequestRead(#[source] axum::Error),
    #[error("failed to process request body: {0}")]
    RequestTooLarge(String),
    // Batch-related, but not specific to any entities
    #[error(transparent)]
    BatchParse(ApiErrorIndexed<serde_json::Error>),
    #[error(transparent)]
    BatchBackrefResolve(ApiErrorIndexed<rs::err::BrResolveError>),
    // Source-related
    #[error("\"{0}\" cannot be used as a source alias")]
    PathSrcParseOnAdd(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("alias \"{0}\" not found")]
    PathSrcParseMisc(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error(transparent)]
    PathSrcNotFound(#[from] rs::src::err::SrcGetError),
    #[error("alias \"{0}\" not found")]
    BodySrcParse(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("EVE data handler not found for requested format \"{0}\"")]
    EdhNotFound(String),
    #[error("EVE data handler initialization failed")]
    EdhInit(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    SrcAdd(#[from] rs::src::err::SrcAddError),
    #[error(transparent)]
    SrcRemove(#[from] rs::src::err::SrcRemoveError),
    // Solar system-related
    #[error(transparent)]
    PathSolParse(#[from] rs::err::ParseSolarSystemIdError),
    #[error(transparent)]
    PathSolNotFound(#[from] rs::err::SolGetError),
    #[error(transparent)]
    SolAdd(#[from] rs::err::SolAddError),
    #[error(transparent)]
    SolChange(#[from] rs::err::SolChangeEnumSolInfoError),
    #[error(transparent)]
    SolRemove(#[from] rs::err::SolRemoveError),
    #[error(transparent)]
    SolSrcSwitch(#[from] rs::err::SolSwitchSrcError),
    #[error(transparent)]
    SolBatchCtl(ApiErrorIndexed<rs::err::SolChangeEnumError>),
    #[error(transparent)]
    SolBatchInfo(ApiErrorIndexed<rs::err::SolInfoEnumError>),
    #[error(transparent)]
    SolBatchVal(ApiErrorIndexed<rs::val::err::SolValEnumError>),
    #[error(transparent)]
    SolBatchTryItems(ApiErrorIndexed<rs::trial::err::SolTryItemsEnumError>),
    // Fleet-related
    #[error(transparent)]
    PathFleetParse(#[from] rs::err::ParseFleetIdError),
    #[error(transparent)]
    PathFleetNotFound(#[from] rs::err::FleetGetError),
    #[error(transparent)]
    FleetAdd(#[from] rs::err::FleetAddError),
    #[error(transparent)]
    FleetChange(#[from] rs::err::FleetChangeError),
    // Fit-related
    #[error(transparent)]
    PathFitParse(#[from] rs::err::ParseFitIdError),
    #[error(transparent)]
    PathFitNotFound(#[from] rs::err::FitGetError),
    #[error(transparent)]
    FitAdd(#[from] rs::err::FitAddError),
    #[error(transparent)]
    FitChange(#[from] rs::err::FitChangeEnumFitInfoError),
    #[error(transparent)]
    FitBatchCtl(ApiErrorIndexed<rs::err::FitChangeEnumError>),
    #[error(transparent)]
    FitBatchInfo(ApiErrorIndexed<rs::err::FitInfoEnumError>),
    // Item-related
    #[error(transparent)]
    PathItemParse(#[from] rs::err::ParseItemIdError),
    #[error(transparent)]
    PathItemNotFound(#[from] rs::err::ItemGetError),
    #[error(transparent)]
    ItemAdd(#[from] rs::err::ItemAddEnumError),
    #[error(transparent)]
    ItemChange(#[from] rs::err::ItemChangeEnumError),
    #[error(transparent)]
    ItemRemove(#[from] rs::err::ItemRemoveError),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmd_index: Option<usize>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<QueryRejection> for ApiError {
    fn from(err: QueryRejection) -> Self {
        Self::Query(err)
    }
}
impl From<JsonRejection> for ApiError {
    fn from(err: JsonRejection) -> Self {
        Self::Json(err)
    }
}
impl From<rs::err::SolHybridBatchError> for ApiError {
    fn from(err: rs::err::SolHybridBatchError) -> Self {
        match err {
            rs::err::SolHybridBatchError::BrResolve(index, inner) => {
                Self::BatchBackrefResolve(ApiErrorIndexed { index, error: inner })
            }
            rs::err::SolHybridBatchError::CtlExec(index, inner) => {
                Self::SolBatchCtl(ApiErrorIndexed { index, error: inner })
            }
            rs::err::SolHybridBatchError::InfoExec(index, inner) => {
                Self::SolBatchInfo(ApiErrorIndexed { index, error: inner })
            }
            rs::err::SolHybridBatchError::ValExec(index, inner) => {
                Self::SolBatchVal(ApiErrorIndexed { index, error: inner })
            }
            rs::err::SolHybridBatchError::TryItemsExec(index, inner) => {
                Self::SolBatchTryItems(ApiErrorIndexed { index, error: inner })
            }
        }
    }
}
impl From<rs::err::FitHybridBatchError> for ApiError {
    fn from(err: rs::err::FitHybridBatchError) -> Self {
        match err {
            rs::err::FitHybridBatchError::BrResolve(index, inner) => {
                Self::BatchBackrefResolve(ApiErrorIndexed { index, error: inner })
            }
            rs::err::FitHybridBatchError::CtlExec(index, inner) => {
                Self::FitBatchCtl(ApiErrorIndexed { index, error: inner })
            }
            rs::err::FitHybridBatchError::InfoExec(index, inner) => {
                Self::FitBatchInfo(ApiErrorIndexed { index, error: inner })
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (http_code, api_code) = self.get_codes();
        let cmd_index = self.get_cmd_index();
        let payload = ApiErrorResponse {
            code: api_code,
            message: std::error::Report::new(&self).to_string(),
            cmd_index,
        };
        (http_code, Json(payload)).into_response()
    }
}
