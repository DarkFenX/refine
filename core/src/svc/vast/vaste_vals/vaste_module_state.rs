use std::collections::HashMap;

use crate::{
    def::{ItemId, ItemKey},
    misc::ModuleState,
    svc::{SvcCtx, vast::VastFitData},
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
    pub(in crate::svc::vast) fn validate_module_state_fast(&self, kfs: &RSet<ItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.mods_state.is_empty(),
            false => self.mods_state.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_module_state_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValModuleStateFail> {
        let modules: HashMap<_, _> = self
            .mods_state
            .iter()
            .filter(|(module_key, _)| !kfs.contains(module_key))
            .map(|(module_key, module_info)| (ctx.uad.items.id_by_key(*module_key), *module_info))
            .collect();
        match modules.is_empty() {
            true => None,
            false => Some(ValModuleStateFail { modules }),
        }
    }
}
