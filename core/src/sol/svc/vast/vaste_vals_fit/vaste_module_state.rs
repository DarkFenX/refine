use crate::{
    defs::SolItemId,
    sol::{svc::vast::SolVastFitData, uad::item::SolModuleState},
    util::StSet,
};

#[derive(Clone)]
pub struct SolValModuleStateFail {
    pub item_id: SolItemId,
    pub state: SolModuleState,
    pub max_state: SolModuleState,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_module_state_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        match kfs.is_empty() {
            true => self.mods_state.is_empty(),
            false => self.mods_state.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_module_state_verbose(
        &self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValModuleStateFail> {
        self.mods_state
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .cloned()
            .collect()
    }
}
