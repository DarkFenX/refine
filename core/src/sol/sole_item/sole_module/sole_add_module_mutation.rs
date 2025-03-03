use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ItemNotMutatedError},
    sol::{SolarSystem, uad::item::SolItemAddMutation},
};

impl SolarSystem {
    pub fn add_module_mutation(
        &mut self,
        item_id: &SolItemId,
        mutation: SolItemAddMutation,
    ) -> Result<(), AddModuleMutationError> {
        let item = self.uad.items.get_item(item_id)?;
        self.svc.unload_item(&self.uad, item);
        let module = match self.uad.items.get_item_mut(item_id).unwrap().get_module_mut() {
            Ok(module) => module,
            Err(error) => {
                let item = self.uad.items.get_item(item_id).unwrap();
                self.svc.load_item(&self.uad, item);
                return Err(error.into());
            }
        };
        if let Err(error) = module.mutate(&self.uad.src, mutation) {
            let item = self.uad.items.get_item(item_id).unwrap();
            self.svc.load_item(&self.uad, item);
            return Err(error.into());
        }
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.load_item(&self.uad, item);
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddModuleMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    MutationAlreadySet(ItemNotMutatedError),
}
impl std::error::Error for AddModuleMutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
            Self::MutationAlreadySet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddModuleMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
            Self::MutationAlreadySet(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for AddModuleMutationError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for AddModuleMutationError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
impl From<ItemNotMutatedError> for AddModuleMutationError {
    fn from(error: ItemNotMutatedError) -> Self {
        Self::MutationAlreadySet(error)
    }
}
