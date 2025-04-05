use std::collections::HashMap;

use crate::{
    sol::{ItemId, svc::vast::VastFitData, uad::item::ModuleState},
    util::RSet,
};

pub struct ValModuleStateFail {
    /// Map between module item IDs and module state info.
    pub modules: HashMap<ItemId, ValModuleStateModuleInfo>,
}
#[derive(Copy, Clone)]
pub struct ValModuleStateModuleInfo {
    /// Current module state.
    pub state: ModuleState,
    /// Highest state this module can be in.
    pub max_state: ModuleState,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_module_state_fast(&self, kfs: &RSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.mods_state.is_empty(),
            false => self.mods_state.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_module_state_verbose(
        &self,
        kfs: &RSet<ItemId>,
    ) -> Option<ValModuleStateFail> {
        let modules: HashMap<_, _> = self
            .mods_state
            .iter()
            .filter(|(module_item_id, _)| !kfs.contains(module_item_id))
            .map(|(module_item_id, module_info)| (*module_item_id, *module_info))
            .collect();
        match modules.is_empty() {
            true => None,
            false => Some(ValModuleStateFail { modules }),
        }
    }
}
