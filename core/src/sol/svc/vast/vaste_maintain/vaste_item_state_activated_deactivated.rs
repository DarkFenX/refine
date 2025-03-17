use crate::{
    defs::OF,
    sol::{
        svc::vast::SolVast,
        uad::item::{SolItem, SolItemState},
    },
};

impl SolVast {
    pub(in crate::sol::svc) fn item_state_activated(&mut self, item: &SolItem, state: &SolItemState) {
        if let SolItemState::Online = state {
            match item {
                SolItem::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
                    let val = match drone.get_a_extras() {
                        Some(extras) => extras.bandwidth_use.unwrap_or(OF(0.0)),
                        None => OF(0.0),
                    };
                    fit_data.drones_online_bandwidth.insert(drone.get_id(), val);
                }
                SolItem::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    fit_data.fighters_online.insert(fighter.get_id());
                }
                _ => (),
            }
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated(&mut self, item: &SolItem, state: &SolItemState) {
        if let SolItemState::Online = state {
            match item {
                SolItem::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
                    fit_data.drones_online_bandwidth.remove(&drone.get_id());
                }
                SolItem::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    fit_data.fighters_online.remove(&fighter.get_id());
                }
                _ => (),
            }
        }
    }
}
