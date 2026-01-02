use std::collections::HashMap;

use crate::{
    api::ModuleState,
    def::ItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
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
    pub(in crate::svc::vast) fn validate_module_state_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.mods_state.is_empty(),
            false => self.mods_state.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_module_state_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValModuleStateFail> {
        let modules: HashMap<_, _> = self
            .mods_state
            .iter()
            .filter(|(module_key, _)| !kfs.contains(module_key))
            .map(|(module_key, module_info)| (ctx.u_data.items.eid_by_iid(*module_key), *module_info))
            .collect();
        match modules.is_empty() {
            true => None,
            false => Some(ValModuleStateFail { modules }),
        }
    }
}
