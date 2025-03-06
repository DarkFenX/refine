use crate::{
    defs::SolItemId,
    sol::{svc::vast::SolVastFitData, uad::item::SolModuleState},
};

#[derive(Clone)]
pub struct SolValModuleStateFail {
    pub item_id: SolItemId,
    pub state: SolModuleState,
    pub max_state: SolModuleState,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_module_state_fast(&self) -> bool {
        self.mods_state.is_empty()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_module_state_verbose(&self) -> Vec<SolValModuleStateFail> {
        self.mods_state.values().cloned().collect()
    }
}
