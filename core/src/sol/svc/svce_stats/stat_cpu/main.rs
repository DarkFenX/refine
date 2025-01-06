use crate::{
    defs::{SolFitId, SolItemId},
    util::StMapSetL1,
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_stats) struct SolStatRegCpu {
    pub(super) items: StMapSetL1<SolFitId, SolItemId>,
}
impl SolStatRegCpu {
    pub(in crate::sol::svc::svce_stats) fn new() -> Self {
        Self {
            items: StMapSetL1::new(),
        }
    }
    pub(in crate::sol::svc::svce_stats) fn register_item(&mut self, fit_id: SolFitId, item_id: SolItemId) {
        self.items.add_entry(fit_id, item_id);
    }
    pub(in crate::sol::svc::svce_stats) fn unregister_item(&mut self, fit_id: &SolFitId, item_id: &SolItemId) {
        self.items.remove_entry(fit_id, item_id);
    }
}
