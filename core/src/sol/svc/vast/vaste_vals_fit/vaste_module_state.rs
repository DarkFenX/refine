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
impl SolValModuleStateFail {
    pub(in crate::sol::svc::vast) fn new(item_id: SolItemId, state: SolModuleState, max_state: SolModuleState) -> Self {
        Self {
            item_id,
            state,
            max_state,
        }
    }
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
