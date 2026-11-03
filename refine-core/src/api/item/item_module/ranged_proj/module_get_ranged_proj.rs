use crate::{ItemId, Module, RangedProj, err::ProjGetError};

impl<'s> Module<'s> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<RangedProj<'_>, ProjGetError> {
        self.sol.internal_get_ranged_proj(self.uid, projectee_item_id)
    }
}
