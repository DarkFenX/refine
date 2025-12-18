use crate::{
    api::{Ability, AbilityMut, Fighter, FighterMut},
    def::AbilId,
    err::basic::AbilityFoundError,
    sol::SolarSystem,
    ud::UItemKey,
};

impl<'a> Fighter<'a> {
    pub fn get_ability(&self, ability_id: &AbilId) -> Result<Ability<'_>, GetAbilityError> {
        check_ability(self.sol, self.key, ability_id)?;
        Ok(Ability::new(self.sol, self.key, *ability_id))
    }
}

impl<'a> FighterMut<'a> {
    pub fn get_ability(&mut self, ability_id: &AbilId) -> Result<Ability<'_>, GetAbilityError> {
        check_ability(self.sol, self.key, ability_id)?;
        Ok(Ability::new(self.sol, self.key, *ability_id))
    }
    pub fn get_ability_mut(&mut self, ability_id: &AbilId) -> Result<AbilityMut<'_>, GetAbilityError> {
        check_ability(self.sol, self.key, ability_id)?;
        Ok(AbilityMut::new(self.sol, self.key, *ability_id))
    }
}

fn check_ability(sol: &SolarSystem, fighter_key: UItemKey, ability_id: &AbilId) -> Result<(), AbilityFoundError> {
    let u_fighter = sol.u_data.items.get(fighter_key).dc_fighter().unwrap();
    if let Some(abils) = u_fighter.get_abils()
        && abils.contains(ability_id)
    {
        return Ok(());
    }
    Err(AbilityFoundError {
        item_id: sol.u_data.items.id_by_key(fighter_key),
        ability_id: *ability_id,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum GetAbilityError {
    #[error("{0}")]
    AbilityNotFound(#[from] AbilityFoundError),
}
