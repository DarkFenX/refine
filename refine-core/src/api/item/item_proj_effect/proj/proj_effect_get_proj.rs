use crate::{ItemId, Proj, ProjEffect, ProjEffectMut, ProjMut, err::ProjGetError};

impl<'s> ProjEffect<'s> {
    pub fn get_proj(&self, projectee_item_id: &ItemId) -> Result<Proj<'_>, ProjGetError> {
        self.sol.internal_get_proj(self.uid, projectee_item_id)
    }
}

impl<'s> ProjEffectMut<'s> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<Proj<'_>, ProjGetError> {
        self.sol.internal_get_proj(self.uid, projectee_item_id)
    }
    pub fn get_proj_mut(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, ProjGetError> {
        self.sol.internal_get_proj_mut(self.uid, projectee_item_id)
    }
}
