use crate::{
    defs::SolItemId,
    err::basic::{ChargeFoundError, ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_module_mutation(&mut self, item_id: &SolItemId) -> Result<(), RemoveModuleMutationError> {
        let module = self.items.get_item(item_id)?.get_module()?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    MutationNotSet(ChargeFoundError),
}
impl std::error::Error for RemoveModuleMutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
            Self::MutationNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveModuleMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
            Self::MutationNotSet(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveModuleMutationError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveModuleMutationError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
