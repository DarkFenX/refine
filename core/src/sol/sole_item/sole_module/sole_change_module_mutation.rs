use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::ItemChangeAttrMutation},
};

impl SolarSystem {
    pub fn change_module_mutation(
        &mut self,
        item_id: &ItemId,
        attr_mutations: Vec<ItemChangeAttrMutation>,
    ) -> Result<(), ChangeModuleMutationError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.change_module_mutation_internal(item_key, attr_mutations)
    }
    pub(in crate::sol) fn change_module_mutation_internal(
        &mut self,
        item_key: ItemKey,
        attr_mutations: Vec<ItemChangeAttrMutation>,
    ) -> Result<(), ChangeModuleMutationError> {
        let module = self.uad.items.get_mut(item_key).get_module_mut()?;
        let changed_a_attr_ids = module.change_mutation_attrs(&self.uad.src, attr_mutations)?;
        for a_attr_id in changed_a_attr_ids {
            self.svc.item_base_attr_value_changed(&self.uad, item_key, a_attr_id);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeModuleMutationError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
    #[error("{0}")]
    MutationNotSet(#[from] ItemMutatedError),
}
