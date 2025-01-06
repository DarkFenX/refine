use crate::{
    defs::{SolFitId, SolItemId},
    sol::item::{SolItem, SolItemState},
    util::StMapSetL1,
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_stats) struct SolStatRegModsOnline {
    pub(super) items: StMapSetL1<SolFitId, SolItemId>,
}
impl SolStatRegModsOnline {
    pub(in crate::sol::svc::svce_stats) fn new() -> Self {
        Self {
            items: StMapSetL1::new(),
        }
    }
    pub(in crate::sol::svc::svce_stats) fn item_state_activated_loaded(
        &mut self,
        item: &SolItem,
        state: &SolItemState,
    ) {
        if matches!(state, SolItemState::Online) {
            if let SolItem::Module(module) = item {
                self.items.add_entry(module.get_fit_id(), module.get_id());
            }
        }
    }
    pub(in crate::sol::svc::svce_stats) fn item_state_deactivated_loaded(
        &mut self,
        item: &SolItem,
        state: &SolItemState,
    ) {
        if matches!(state, SolItemState::Online) {
            if let SolItem::Module(module) = item {
                self.items.remove_entry(&module.get_fit_id(), &module.get_id());
            }
        }
    }
}
