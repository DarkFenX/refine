use crate::{
    Value,
    rd::RState,
    svc::Vast,
    ud::{UItem, UItemId},
};

impl Vast {
    pub(in crate::svc) fn item_state_activated(&mut self, item_uid: UItemId, item: &UItem, state: RState) {
        if let RState::Online = state {
            match item {
                UItem::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(drone.get_fit_uid());
                    let val = match drone.get_r_item_attr_data() {
                        Some(drone_riad) => drone_riad.bandwidth_use.unwrap_or(Value::ZERO),
                        None => Value::ZERO,
                    };
                    fit_data.drones_online_bandwidth.insert(item_uid, val);
                }
                UItem::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(fighter.get_fit_uid());
                    fit_data.fighters_online.insert(item_uid);
                }
                _ => (),
            }
        }
    }
    pub(in crate::svc) fn item_state_deactivated(&mut self, item_uid: &UItemId, item: &UItem, state: RState) {
        if let RState::Online = state {
            match item {
                UItem::Drone(drone) => {
                    let fit_data = self.get_fit_data_mut(drone.get_fit_uid());
                    fit_data.drones_online_bandwidth.remove(item_uid);
                }
                UItem::Fighter(fighter) => {
                    let fit_data = self.get_fit_data_mut(fighter.get_fit_uid());
                    fit_data.fighters_online.remove(item_uid);
                }
                _ => (),
            }
        }
    }
}
