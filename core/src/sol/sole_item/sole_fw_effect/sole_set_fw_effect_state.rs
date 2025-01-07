use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_fw_effect_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetFwEffectStateError> {
        let fw_effect = self.uad.items.get_item_mut(item_id)?.get_fw_effect_mut()?;
        let old_state = fw_effect.get_state();
        fw_effect.set_bool_state(state);
        let new_state = fw_effect.get_state();
        self.change_item_id_state_in_svc(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFwEffectStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFwEffect(ItemKindMatchError),
}
impl std::error::Error for SetFwEffectStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFwEffectStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetFwEffectStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetFwEffectStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFwEffect(error)
    }
}
