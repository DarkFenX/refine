use crate::sol::{
    ItemId,
    api::{Drone, DroneMut, GetRangedProjError, RangedProj, RangedProjMut},
};

impl<'a> Drone<'a> {
    pub fn get_proj(&self, projectee_item_id: &ItemId) -> Result<RangedProj, GetRangedProjError> {
        let projectee_item_key = self
            .sol
            .internal_get_ranged_projectee_item_key(self.key, projectee_item_id)?;
        Ok(RangedProj::new(self.sol, self.key, projectee_item_key))
    }
}

impl<'a> DroneMut<'a> {
    pub fn get_proj(&mut self, projectee_item_id: &ItemId) -> Result<RangedProj, GetRangedProjError> {
        let projectee_item_key = self
            .sol
            .internal_get_ranged_projectee_item_key(self.key, projectee_item_id)?;
        Ok(RangedProj::new(self.sol, self.key, projectee_item_key))
    }
    pub fn get_proj_mut(&mut self, projectee_item_id: &ItemId) -> Result<RangedProjMut, GetRangedProjError> {
        let projectee_item_key = self
            .sol
            .internal_get_ranged_projectee_item_key(self.key, projectee_item_id)?;
        Ok(RangedProjMut::new(self.sol, self.key, projectee_item_key))
    }
}
