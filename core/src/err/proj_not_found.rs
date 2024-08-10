use std::{error, fmt};

use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ProjNotFoundError {
    pub projector_item_id: SolItemId,
    pub projectee_item_id: SolItemId,
}
impl ProjNotFoundError {
    pub(crate) fn new(projector_item_id: SolItemId, projectee_item_id: SolItemId) -> Self {
        Self {
            projector_item_id,
            projectee_item_id,
        }
    }
}
impl error::Error for ProjNotFoundError {}
impl fmt::Display for ProjNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "projection {}->{} is already defined",
            self.projector_item_id, self.projectee_item_id
        )
    }
}
