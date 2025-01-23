use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_module_mutation(&mut self, item_id: &SolItemId) -> Result<(), RemoveModuleMutationError> {
        let item = self.uad.items.get_item(item_id)?;
        let module = item.get_module()?;
        if !module.has_mutation_data() {
            return Err(ItemMutatedError::new(*item_id).into());
        }
        self.svc.unload_item(&self.uad, item);
        self.uad
            .items
            .get_item_mut(item_id)
            .unwrap()
            .get_module_mut()
            .unwrap()
            .unmutate(&self.uad.src)
            .unwrap();
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.load_item(&self.uad, item);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    MutationNotSet(ItemMutatedError),
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
impl From<ItemMutatedError> for RemoveModuleMutationError {
    fn from(error: ItemMutatedError) -> Self {
        Self::MutationNotSet(error)
    }
}
