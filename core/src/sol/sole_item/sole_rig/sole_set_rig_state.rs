use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn set_rig_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetRigStateError> {
        let rig = self.items.get_item_mut(item_id)?.get_rig_mut()?;
        let old_state = rig.state;
        rig.set_bool_state(state);
        let new_state = rig.state;
        self.change_item_id_state_in_svcs(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetRigStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotRig(ItemKindMatchError),
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
