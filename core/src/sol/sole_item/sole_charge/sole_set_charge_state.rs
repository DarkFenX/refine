use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_charge_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetChargeStateError> {
        let charge = self.uad.items.get_item_mut(item_id)?.get_charge_mut()?;
        let old_state = charge.get_state();
        charge.set_force_disable(!state);
        let new_state = charge.get_state();
        self.change_item_id_state_in_svcs(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetChargeStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharge(ItemKindMatchError),
}
impl std::error::Error for SetChargeStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotCharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetChargeStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotCharge(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetChargeStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetChargeStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotCharge(error)
    }
}
