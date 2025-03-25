use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn set_proj_effect_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetProjEffectStateError> {
        let proj_effect = self.uad.items.get_item_mut(item_id)?.get_proj_effect_mut()?;
        let old_a_state = proj_effect.get_a_state();
        proj_effect.set_proj_effect_state(state);
        let new_a_state = proj_effect.get_a_state();
        self.change_item_id_state_in_svc(item_id, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetProjEffectStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotProjEffect(ItemKindMatchError),
}
impl std::error::Error for SetProjEffectStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotProjEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetProjEffectStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotProjEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetProjEffectStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetProjEffectStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotProjEffect(error)
    }
}
