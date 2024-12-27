use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_module_mutation(&mut self, item_id: &SolItemId) -> Result<(), RemoveModuleMutationError> {
        let item = self.items.get_item(item_id)?;
        self.svcs.unload_item(
            &SolView::new(
                &self.src,
                &self.fleets,
                &self.fits,
                &self.items,
                &self.default_incoming_dmg,
            ),
            item,
        );
        let module = match self.items.get_item_mut(item_id).unwrap().get_module_mut() {
            Ok(module) => module,
            Err(error) => {
                let item = self.items.get_item(item_id).unwrap();
                self.svcs.load_item(
                    &SolView::new(
                        &self.src,
                        &self.fleets,
                        &self.fits,
                        &self.items,
                        &self.default_incoming_dmg,
                    ),
                    item,
                );
                return Err(error.into());
            }
        };
        if let Err(error) = module.unmutate(&self.src) {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.load_item(
                &SolView::new(
                    &self.src,
                    &self.fleets,
                    &self.fits,
                    &self.items,
                    &self.default_incoming_dmg,
                ),
                item,
            );
            return Err(error.into());
        }
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.load_item(
            &SolView::new(
                &self.src,
                &self.fleets,
                &self.fits,
                &self.items,
                &self.default_incoming_dmg,
            ),
            item,
        );
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
