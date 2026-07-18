#[derive(serde::Deserialize)]
pub(crate) struct SolInfoParams {
    pub(super) sol: Option<rs::SolInfoMode>,
    pub(super) fleet: Option<rs::FleetInfoMode>,
    pub(super) fit: Option<rs::FitInfoMode>,
    pub(super) item: Option<rs::ItemInfoMode>,
}
