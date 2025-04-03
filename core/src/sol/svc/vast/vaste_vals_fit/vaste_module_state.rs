use crate::{
    sol::{ItemId, svc::vast::VastFitData, uad::item::ModuleState},
    util::HSet,
};

#[derive(Clone)]
pub struct ValModuleStateFail {
    pub item_id: ItemId,
    pub state: ModuleState,
    pub max_state: ModuleState,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_module_state_fast(&self, kfs: &HSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.mods_state.is_empty(),
            false => self.mods_state.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_module_state_verbose(
        &self,
        kfs: &HSet<ItemId>,
    ) -> Vec<ValModuleStateFail> {
        self.mods_state
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .cloned()
            .collect()
    }
}
