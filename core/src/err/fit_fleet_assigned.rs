use std::{error, fmt};

use crate::defs::SolFitId;

#[derive(Debug)]
pub struct FitFleetAssignedError {
    pub fit_id: SolFitId,
}
impl FitFleetAssignedError {
    pub(crate) fn new(fit_id: SolFitId) -> Self {
        Self { fit_id }
    }
}
impl error::Error for FitFleetAssignedError {}
impl fmt::Display for FitFleetAssignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fit {} does not belong to any fleet", self.fit_id)
    }
}
