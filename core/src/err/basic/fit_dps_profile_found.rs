use crate::sol::FitId;

#[derive(thiserror::Error, Debug)]
#[error("DPS profile not found on fit {fit_id}")]
pub struct FitDpsProfileFoundError {
    pub fit_id: FitId,
}
