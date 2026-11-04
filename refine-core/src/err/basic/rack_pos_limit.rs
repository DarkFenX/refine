use crate::{FitId, ModRack, RackPos};

#[derive(Clone, Debug, thiserror::Error)]
#[error("{rack} rack of fit {fit_id} can't have modules past position {}", RackPos::MAX)]
pub struct RackPosLimitError {
    pub fit_id: FitId,
    pub rack: ModRack,
}
