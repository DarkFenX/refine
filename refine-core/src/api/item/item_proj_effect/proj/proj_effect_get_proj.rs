use crate::{ItemId, Proj, ProjEffect, err::ProjGetError};

impl<'s> ProjEffect<'s> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<Proj<'_>, ProjGetError> {
        self.sol.internal_get_proj(self.uid, projectee_item_id)
    }
}
