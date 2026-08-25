use crate::{Module, ModuleMut, RangedProj, RangedProjMut, err::ProjGetError, ud::ItemId};

impl<'s> Module<'s> {
    pub fn get_proj(&self, projectee_item_id: &ItemId) -> Result<RangedProj<'_>, ProjGetError> {
        self.sol.internal_get_ranged_proj(self.uid, projectee_item_id)
    }
}

impl<'s> ModuleMut<'s> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<RangedProj<'_>, ProjGetError> {
        self.sol.internal_get_ranged_proj(self.uid, projectee_item_id)
    }
    pub fn get_proj_mut(&mut self, projectee_item_id: &ItemId) -> Result<RangedProjMut<'_>, ProjGetError> {
        self.sol.internal_get_ranged_proj_mut(self.uid, projectee_item_id)
    }
}
