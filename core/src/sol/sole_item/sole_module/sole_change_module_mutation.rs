use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{item::SolItemChangeAttrMutation, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn change_module_mutation(
        &mut self,
        item_id: &SolItemId,
        attr_mutations: Vec<SolItemChangeAttrMutation>,
    ) -> Result<(), ChangeModuleMutationError> {
        let module = self.items.get_item_mut(item_id)?.get_module_mut()?;
        let changed_attr_ids = module.change_mutation_attrs(&self.src, attr_mutations)?;
        let sol_view = SolView::new(
            &self.src,
            &self.fleets,
            &self.fits,
            &self.items,
            &self.default_incoming_dmg,
        );
        for attr_id in changed_attr_ids {
            self.svcs.item_attr_value_changed(&sol_view, item_id, &attr_id);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ChangeModuleMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    MutationNotSet(ItemMutatedError),
}
impl std::error::Error for ChangeModuleMutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
            Self::MutationNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for ChangeModuleMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
            Self::MutationNotSet(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for ChangeModuleMutationError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for ChangeModuleMutationError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
impl From<ItemMutatedError> for ChangeModuleMutationError {
    fn from(error: ItemMutatedError) -> Self {
        Self::MutationNotSet(error)
    }
}
