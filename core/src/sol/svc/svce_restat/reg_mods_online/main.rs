use crate::{
    defs::SolItemId,
    sol::item::{SolItem, SolItemState},
    util::StSet,
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_restat) struct SolRestatRegModsOnline {
    pub(super) items: StSet<SolItemId>,
}
impl SolRestatRegModsOnline {
    pub(in crate::sol::svc::svce_restat) fn new() -> Self {
        Self { items: StSet::new() }
    }
    pub(in crate::sol::svc::svce_restat) fn item_state_activated_loaded(
        &mut self,
        item: &SolItem,
        state: &SolItemState,
    ) {
        if matches!(state, SolItemState::Online) {
            if let SolItem::Module(module) = item {
                self.items.insert(module.get_id());
            }
        }
    }
    pub(in crate::sol::svc::svce_restat) fn item_state_deactivated_loaded(
        &mut self,
        item: &SolItem,
        state: &SolItemState,
    ) {
        if matches!(state, SolItemState::Online) {
            if let SolItem::Module(module) = item {
                self.items.remove(&module.get_id());
            }
        }
    }
}
