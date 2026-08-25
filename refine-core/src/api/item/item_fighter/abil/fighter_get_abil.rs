use crate::{Ability, AbilityId, AbilityMut, Fighter, FighterMut, ItemId, SolarSystem, ud::UItemId};

impl<'s> Fighter<'s> {
    pub fn get_ability(&self, ability_id: &AbilityId) -> Result<Ability<'_>, AbilityGetError> {
        check_ability(self.sol, self.uid, ability_id)?;
        Ok(Ability::new(self.sol, self.uid, ability_id.into_aid()))
    }
}

impl<'s> FighterMut<'s> {
    pub fn get_ability(&mut self, ability_id: &AbilityId) -> Result<Ability<'_>, AbilityGetError> {
        check_ability(self.sol, self.uid, ability_id)?;
        Ok(Ability::new(self.sol, self.uid, ability_id.into_aid()))
    }
    pub fn get_ability_mut(&mut self, ability_id: &AbilityId) -> Result<AbilityMut<'_>, AbilityGetError> {
        check_ability(self.sol, self.uid, ability_id)?;
        Ok(AbilityMut::new(self.sol, self.uid, ability_id.into_aid()))
    }
}

fn check_ability(sol: &SolarSystem, fighter_uid: UItemId, ability_id: &AbilityId) -> Result<(), AbilityGetError> {
    let u_fighter = sol.u_data.items.get(fighter_uid).dc_fighter().unwrap();
    if let Some(rib) = u_fighter.get_r_item_base()
        && rib.abil_ids.contains(&ability_id.into_aid())
    {
        return Ok(());
    }
    Err(AbilityGetError::AbilityNotFound(
        sol.u_data.items.ext_id_by_int_id(fighter_uid),
        *ability_id,
    ))
}

#[derive(Debug, thiserror::Error)]
pub enum AbilityGetError {
    #[error("ability {1} is not found on item {0}")]
    AbilityNotFound(ItemId, AbilityId),
}
