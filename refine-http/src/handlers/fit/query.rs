use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FitInfoParams {
    pub(super) fit: Option<rs::FitInfoMode>,
    pub(super) item: Option<rs::ItemInfoMode>,
}
