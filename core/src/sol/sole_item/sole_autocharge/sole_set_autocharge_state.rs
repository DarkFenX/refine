use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_autocharge_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetAutochargeStateError> {
        let autocharge = self.uad.items.get_item_mut(item_id)?.get_autocharge_mut()?;
        let old_state = autocharge.get_state();
        autocharge.set_force_disable(!state);
        let new_state = autocharge.get_state();
        self.change_item_id_state_in_svcs(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetAutochargeStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotAutocharge(ItemKindMatchError),
}
impl std::error::Error for SetAutochargeStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotAutocharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetAutochargeStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotAutocharge(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetAutochargeStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetAutochargeStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotAutocharge(error)
    }
}
