use std::{error, fmt};

use crate::defs::SolFitId;

#[derive(Debug)]
pub struct FitHasItemKindError {
    pub fit_id: SolFitId,
    pub item_kind: &'static str,
}
impl FitHasItemKindError {
    pub(crate) fn new(fit_id: SolFitId, item_kind: &'static str) -> Self {
        Self { fit_id, item_kind }
    }
}
impl error::Error for FitHasItemKindError {}
impl fmt::Display for FitHasItemKindError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fit {} does not have {} set", self.fit_id, self.item_kind)
    }
}
