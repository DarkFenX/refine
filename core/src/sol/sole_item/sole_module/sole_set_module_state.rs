use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem, uad::item::ModuleState},
};

impl SolarSystem {
    pub fn set_module_state(&mut self, item_id: &ItemId, state: ModuleState) -> Result<(), SetModuleStateError> {
        // Update user data for module
        let module = self.uad.items.get_mut_by_id(item_id)?.get_module_mut()?;
        let charge_id = module.get_charge_item_id();
        let old_a_state = module.get_a_state();
        module.set_module_state(state);
        // Update services for module
        let new_a_state = module.get_a_state();
        self.change_item_id_state_in_svc(item_id, old_a_state, new_a_state);
        if let Some(charge_id) = charge_id {
            // Update user data for charge
            let charge = self
                .uad
                .items
                .get_mut_by_id(&charge_id)
                .unwrap()
                .get_charge_mut()
                .unwrap();
            let old_a_state = charge.get_a_state();
            charge.set_a_state(state.into());
            // Update services for charge
            let new_a_state = charge.get_a_state();
            self.change_item_id_state_in_svc(&charge_id, old_a_state, new_a_state);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetModuleStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
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
