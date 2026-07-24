use crate::ud::FitId;

#[derive(Debug, thiserror::Error)]
#[error("DPS profile not found on fit {fit_id}")]
pub struct FitDpsProfileFoundError {
    pub fit_id: FitId,
}
