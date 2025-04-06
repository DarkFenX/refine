use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn set_rig_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetRigStateError> {
        let rig = self.uad.items.get_mut_by_id(item_id)?.get_rig_mut()?;
        let old_a_state = rig.get_a_state();
        rig.set_rig_state(state);
        let new_a_state = rig.get_a_state();
        self.change_item_id_state_in_svc(item_id, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetRigStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotRig(ItemKindMatchError),
}
impl std::error::Error for SetRigStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotRig(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetRigStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotRig(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetRigStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetRigStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotRig(error)
    }
}
