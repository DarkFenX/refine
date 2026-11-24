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
    PathSolParse(#[from] rs::err::SolIdParseError),
    #[error(transparent)]
    PathSolNotFound(#[from] rs::err::SolGetError),
    #[error(transparent)]
    SolAdd(#[from] rs::ctl::err::SolAddError),
    #[error(transparent)]
    SolChange(#[from] rs::ctl::err::SolChangeEnumError),
    #[error(transparent)]
    SolRemove(#[from] rs::ctl::err::SolRemoveError),
    #[error(transparent)]
    SolSrcSwitch(#[from] rs::ctl::err::SolSwitchSrcError),
    #[error(transparent)]
    SolBatchCtl(ApiErrorIndexed<rs::ctl::err::SolChangeEnumError>),
    #[error(transparent)]
    SolBatchInfo(ApiErrorIndexed<rs::info::err::SolInfoEnumError>),
    #[error(transparent)]
    SolBatchStats(ApiErrorIndexed<rs::stats::err::SolStatsEnumError>),
    #[error(transparent)]
    SolBatchVal(ApiErrorIndexed<rs::val::err::SolValEnumError>),
    #[error(transparent)]
    SolBatchTryItems(ApiErrorIndexed<rs::trial::err::SolTryItemsEnumError>),
    // Fleet-related
    #[error(transparent)]
    PathFleetParse(#[from] rs::err::core::FleetIdParseError),
    #[error(transparent)]
    PathFleetNotFound(#[from] rs::err::FleetGetError),
    #[error(transparent)]
    FleetAdd(#[from] rs::ctl::err::FleetAddError),
    #[error(transparent)]
    FleetChange(#[from] rs::ctl::err::FleetChangeError),
    // Fit-related
    #[error(transparent)]
    PathFitParse(#[from] rs::err::core::FitIdParseError),
    #[error(transparent)]
    PathFitNotFound(#[from] rs::err::FitGetError),
    #[error(transparent)]
    FitAdd(#[from] rs::ctl::err::FitAddError),
    #[error(transparent)]
    FitChange(#[from] rs::ctl::err::FitChangeEnumError),
    #[error(transparent)]
    FitBatchCtl(ApiErrorIndexed<rs::ctl::err::FitChangeEnumError>),
    #[error(transparent)]
    FitBatchInfo(ApiErrorIndexed<rs::info::err::FitInfoEnumError>),
    #[error(transparent)]
    FitBatchStats(ApiErrorIndexed<rs::stats::err::FitStatsEnumError>),
    // Item-related
    #[error(transparent)]
    PathItemParse(#[from] rs::err::core::ItemIdParseError),
    #[error(transparent)]
    PathItemNotFound(#[from] rs::err::ItemGetError),
    #[error(transparent)]
    ItemAdd(#[from] rs::ctl::err::ItemAddEnumError),
    #[error(transparent)]
    ItemChange(#[from] rs::ctl::err::ItemChangeEnumError),
    #[error(transparent)]
    ItemRemove(#[from] rs::ctl::err::ItemRemoveError),
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
impl From<rs::hyb::err::SolHybridBatchError> for ApiError {
    fn from(err: rs::hyb::err::SolHybridBatchError) -> Self {
        match err {
            rs::hyb::err::SolHybridBatchError::BrResolve(index, inner) => {
                Self::BatchBackrefResolve(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::SolHybridBatchError::CtlExec(index, inner) => {
                Self::SolBatchCtl(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::SolHybridBatchError::InfoExec(index, inner) => {
                Self::SolBatchInfo(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::SolHybridBatchError::StatsExec(index, inner) => {
                Self::SolBatchStats(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::SolHybridBatchError::ValExec(index, inner) => {
                Self::SolBatchVal(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::SolHybridBatchError::TryItemsExec(index, inner) => {
                Self::SolBatchTryItems(ApiErrorIndexed { index, error: inner })
            }
        }
    }
}
impl From<rs::hyb::err::FitHybridBatchError> for ApiError {
    fn from(err: rs::hyb::err::FitHybridBatchError) -> Self {
        match err {
            rs::hyb::err::FitHybridBatchError::BrResolve(index, inner) => {
                Self::BatchBackrefResolve(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::FitHybridBatchError::CtlExec(index, inner) => {
                Self::FitBatchCtl(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::FitHybridBatchError::InfoExec(index, inner) => {
                Self::FitBatchInfo(ApiErrorIndexed { index, error: inner })
            }
            rs::hyb::err::FitHybridBatchError::StatsExec(index, inner) => {
                Self::FitBatchStats(ApiErrorIndexed { index, error: inner })
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
