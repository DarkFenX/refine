use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolarSystem, uad::item::SolModuleState},
};

impl SolarSystem {
    pub fn set_module_state(&mut self, item_id: &SolItemId, state: SolModuleState) -> Result<(), SetModuleStateError> {
        // Update user data for module
        let module = self.uad.items.get_item_mut(item_id)?.get_module_mut()?;
        let charge_id = module.get_charge_id();
        let old_state = module.get_state();
        module.set_module_state(state);
        // Update services for module
        let new_state = module.get_state();
        self.change_item_id_state_in_svc(item_id, old_state, new_state);
        if let Some(charge_id) = charge_id {
            // Update user data for charge
            let charge = self
                .uad
                .items
                .get_item_mut(&charge_id)
                .unwrap()
                .get_charge_mut()
                .unwrap();
            let old_state = charge.get_state();
            charge.set_state(state.into());
            // Update services for charge
            let new_state = charge.get_state();
            self.change_item_id_state_in_svc(&charge_id, old_state, new_state);
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
