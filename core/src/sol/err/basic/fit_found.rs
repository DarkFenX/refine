use std::{error, fmt};

use crate::defs::SolFitId;

#[derive(Debug)]
pub struct FitFoundError {
    pub fit_id: SolFitId,
}
impl FitFoundError {
    pub(crate) fn new(fit_id: SolFitId) -> Self {
        Self { fit_id }
    }
}
impl error::Error for FitFoundError {}
impl fmt::Display for FitFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fit {} not found", self.fit_id)
    }
}
