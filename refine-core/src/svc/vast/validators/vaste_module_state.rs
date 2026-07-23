use itertools::Itertools;

use crate::{
    ItemId, ModuleState,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
};

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct ValModuleStateModuleStored {
    pub(in crate::svc::vast) state: ModuleState,
    pub(in crate::svc::vast) max_state: ModuleState,
}

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValModuleStateFail {
    /// Modules and their state info.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub modules: Vec<ValModuleStateModuleInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValModuleStateModuleInfo {
    #[cfg_attr(feature = "serde", serde(rename = "$key$"))]
    pub module_id: ItemId,
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
        let modules = self
            .mods_state
            .iter()
            .filter_map(|(module_uid, module_info)| match kfs.contains(module_uid) {
                true => None,
                false => Some(ValModuleStateModuleInfo {
                    module_id: ctx.u_data.items.ext_id_by_int_id(*module_uid),
                    state: module_info.state,
                    max_state: module_info.max_state,
                }),
            })
            .collect_vec();
        match modules.is_empty() {
            true => None,
            false => Some(ValModuleStateFail { modules }),
        }
    }
}
