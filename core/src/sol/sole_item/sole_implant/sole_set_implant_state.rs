use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_implant_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetImplantStateError> {
        let implant = self.items.get_item_mut(item_id)?.get_implant_mut()?;
        let old_state = implant.state;
        implant.set_bool_state(state);
        let new_state = implant.state;
        self.change_item_id_state_in_svcs(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetImplantStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotImplant(ItemKindMatchError),
}
impl std::error::Error for SetImplantStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotImplant(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetImplantStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotImplant(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetImplantStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetImplantStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotImplant(error)
    }
}
