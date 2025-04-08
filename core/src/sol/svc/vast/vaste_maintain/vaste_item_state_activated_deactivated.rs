use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{ItemKey, svc::vast::Vast, uad::item::Item},
};

impl Vast {
    pub(in crate::sol::svc) fn item_state_activated(&mut self, item_key: ItemKey, item: &Item, a_state: &ad::AState) {
        if let ad::AState::Online = a_state {
            match item {
                Item::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
                    let val = match drone.get_a_extras() {
                        Some(extras) => extras.bandwidth_use.unwrap_or(OF(0.0)),
                        None => OF(0.0),
                    };
                    fit_data.drones_online_bandwidth.insert(item_key, val);
                }
                Item::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    fit_data.fighters_online.insert(item_key);
                }
                _ => (),
            }
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated(
        &mut self,
        item_key: &ItemKey,
        item: &Item,
        a_state: &ad::AState,
    ) {
        if let ad::AState::Online = a_state {
            match item {
                Item::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
                    fit_data.drones_online_bandwidth.remove(item_key);
                }
                Item::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    fit_data.fighters_online.remove(item_key);
                }
                _ => (),
            }
        }
    }
}
