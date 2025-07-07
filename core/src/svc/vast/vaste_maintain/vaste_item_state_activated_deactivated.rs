use crate::{
    ad,
    def::{ItemKey, OF},
    svc::vast::Vast,
    uad::UadItem,
};

impl Vast {
    pub(in crate::svc) fn item_state_activated(&mut self, item_key: ItemKey, item: &UadItem, a_state: &ad::AState) {
        if let ad::AState::Online = a_state {
            match item {
                UadItem::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_key());
                    let val = match drone.get_a_xt() {
                        Some(a_item_xt) => a_item_xt.bandwidth_use.unwrap_or(OF(0.0)),
                        None => OF(0.0),
                    };
                    fit_data.drones_online_bandwidth.insert(item_key, val);
                }
                UadItem::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_key());
                    fit_data.fighters_online.insert(item_key);
                }
                _ => (),
            }
        }
    }
    pub(in crate::svc) fn item_state_deactivated(&mut self, item_key: &ItemKey, item: &UadItem, a_state: &ad::AState) {
        if let ad::AState::Online = a_state {
            match item {
                UadItem::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_key());
                    fit_data.drones_online_bandwidth.remove(item_key);
                }
                UadItem::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_key());
                    fit_data.fighters_online.remove(item_key);
                }
                _ => (),
            }
        }
    }
}
