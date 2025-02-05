use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn set_sw_effect_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetSwEffectStateError> {
        let sw_effect = self.uad.items.get_item_mut(item_id)?.get_sw_effect_mut()?;
        let old_state = sw_effect.get_state();
        sw_effect.set_sw_effect_state(state);
        let new_state = sw_effect.get_state();
        self.change_item_id_state_in_svc(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetSwEffectStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSwEffect(ItemKindMatchError),
}
impl std::error::Error for SetSwEffectStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetSwEffectStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetSwEffectStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetSwEffectStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSwEffect(error)
    }
}
