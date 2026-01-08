use crate::{
    api::{AbilId, Ability, AbilityMut, Fighter, FighterMut},
    err::basic::AbilityFoundError,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> Fighter<'a> {
    pub fn get_ability(&self, ability_id: &AbilId) -> Result<Ability<'_>, GetAbilityError> {
        check_ability(self.sol, self.uid, ability_id)?;
        Ok(Ability::new(self.sol, self.uid, ability_id.into_aid()))
    }
}

impl<'a> FighterMut<'a> {
    pub fn get_ability(&mut self, ability_id: &AbilId) -> Result<Ability<'_>, GetAbilityError> {
        check_ability(self.sol, self.uid, ability_id)?;
        Ok(Ability::new(self.sol, self.uid, ability_id.into_aid()))
    }
    pub fn get_ability_mut(&mut self, ability_id: &AbilId) -> Result<AbilityMut<'_>, GetAbilityError> {
        check_ability(self.sol, self.uid, ability_id)?;
        Ok(AbilityMut::new(self.sol, self.uid, ability_id.into_aid()))
    }
}

fn check_ability(sol: &SolarSystem, fighter_uid: UItemId, ability_id: &AbilId) -> Result<(), AbilityFoundError> {
    let u_fighter = sol.u_data.items.get(fighter_uid).dc_fighter().unwrap();
    if let Some(abils) = u_fighter.get_abils()
        && abils.contains(&ability_id.into_aid())
    {
        return Ok(());
    }
    Err(AbilityFoundError {
        item_id: sol.u_data.items.xid_by_iid(fighter_uid),
        ability_id: *ability_id,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum GetAbilityError {
    #[error("{0}")]
    AbilityNotFound(#[from] AbilityFoundError),
}
