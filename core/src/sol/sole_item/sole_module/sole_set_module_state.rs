use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        item::SolItemState,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn set_module_state(&mut self, item_id: &SolItemId, state: SolItemState) -> Result<(), SetModuleStateError> {
        let module = self.items.get_item_mut(item_id)?.get_module_mut()?;
        let old_state = module.state;
        module.state = state;
        self.change_item_id_state_in_svcs(item_id, old_state, state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetModuleStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
}
impl From<ItemFoundError> for SetModuleStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetModuleStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
impl std::error::Error for SetModuleStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetModuleStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
        }
    }
}
