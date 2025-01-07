use crate::{
    ad,
    defs::SolFitId,
    ec,
    sol::{
        svc::rest::{SolRest, SolRestFitData},
        uad::item::{SolItem, SolItemState},
    },
};

impl SolRest {
    pub(in crate::sol::svc) fn fit_added(&mut self, fit_id: &SolFitId) {
        self.data.insert(*fit_id, SolRestFitData::new());
    }
    pub(in crate::sol::svc) fn fit_removed(&mut self, fit_id: &SolFitId) {
        self.data.remove(fit_id);
    }
    pub(in crate::sol::svc) fn item_loaded(&mut self, item: &SolItem) {}
    pub(in crate::sol::svc) fn item_unloaded(&mut self, item: &SolItem) {}
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {}
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {}
    pub(in crate::sol::svc) fn effects_started(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        if let SolItem::Module(module) = item {
            for effect in effects {
                if effect.id == ec::effects::ONLINE {
                    let fit_data = self.get_data_mut(&module.get_fit_id()).unwrap();
                    fit_data.mods_online.insert(module.get_id());
                }
            }
        }
    }
    pub(in crate::sol::svc) fn effects_stopped(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        if let SolItem::Module(module) = item {
            for effect in effects {
                if effect.id == ec::effects::ONLINE {
                    let fit_data = self.get_data_mut(&module.get_fit_id()).unwrap();
                    fit_data.mods_online.remove(&module.get_id());
                }
            }
        }
    }
}
