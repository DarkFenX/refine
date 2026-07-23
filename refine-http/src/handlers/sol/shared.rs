use crate::err::ApiError;

#[derive(serde::Deserialize)]
pub(crate) struct SolInfoParams {
    pub(super) sol: Option<rs::SolInfoMode>,
    pub(super) fleet: Option<rs::FleetInfoMode>,
    pub(super) fit: Option<rs::FitInfoMode>,
    pub(super) item: Option<rs::ItemInfoMode>,
}

pub(super) fn parse_src_alias_from_body(src_alias: Option<String>) -> Result<Option<rs::src::SrcAlias>, ApiError> {
    match src_alias {
        Some(src_alias) => match rs::src::SrcAlias::try_pruned(&src_alias) {
            Some(src_alias) => Ok(Some(src_alias)),
            None => Err(ApiError::BodySrcParseFailed(src_alias)),
        },
        None => Ok(None),
    }
}
