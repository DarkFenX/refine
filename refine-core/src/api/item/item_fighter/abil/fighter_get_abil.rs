use crate::{Ability, AbilityId, Fighter, ItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fighter<'s> {
    pub fn get_ability(&mut self, ability_id: &AbilityId) -> Result<Ability<'_>, AbilityGetError> {
        let u_fighter = self.sol.u_data.items.get(self.uid).dc_fighter().unwrap();
        if let Some(rib) = u_fighter.get_r_item_base()
            && rib.abil_ids.contains(&ability_id.into_aid())
        {
            return Ok(Ability::new(self.sol, self.uid, ability_id.into_aid()));
        }
        Err(AbilityGetError::AbilityNotFound(
            self.sol.u_data.items.ext_id_by_int_id(self.uid),
            *ability_id,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AbilityGetError {
    #[error("ability {1} is not found on item {0}")]
    AbilityNotFound(ItemId, AbilityId),
}
