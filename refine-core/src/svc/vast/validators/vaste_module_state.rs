use itertools::Itertools;

use crate::{
    api::ModuleState,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValModuleStateFail {
    /// Modules and their state info.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub modules: Vec<(ItemId, ValModuleStateModuleInfo)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
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
        let modules = self
            .mods_state
            .iter()
            .filter(|(module_uid, _)| !kfs.contains(module_uid))
            .map(|(module_uid, module_info)| (ctx.u_data.items.ext_id_by_int_id(*module_uid), *module_info))
            .collect_vec();
        match modules.is_empty() {
            true => None,
            false => Some(ValModuleStateFail { modules }),
        }
    }
}
