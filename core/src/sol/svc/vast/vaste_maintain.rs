use crate::{
    ad,
    defs::{SolFitId, OF},
    ec,
    sol::{
        svc::vast::{SolVast, SolVastFitData},
        uad::item::{SolItem, SolItemState},
    },
};

impl SolVast {
    pub(in crate::sol::svc) fn fit_added(&mut self, fit_id: &SolFitId) {
        self.fit_datas.insert(*fit_id, SolVastFitData::new());
    }
    pub(in crate::sol::svc) fn fit_removed(&mut self, fit_id: &SolFitId) {
        self.fit_datas.remove(fit_id);
    }
    pub(in crate::sol::svc) fn item_loaded(&mut self, item: &SolItem) {
        if let SolItem::Drone(drone) = item {
            if let Some(val) = drone.get_attrs().unwrap().get(&ec::attrs::VOLUME) {
                let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
                fit_data.drones_volume.insert(drone.get_id(), *val);
            }
        }
    }
    pub(in crate::sol::svc) fn item_unloaded(&mut self, item: &SolItem) {
        if let SolItem::Drone(drone) = item {
            let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
            fit_data.drones_volume.remove(&drone.get_id());
        }
    }
    pub(in crate::sol::svc) fn item_state_activated(&mut self, item: &SolItem, state: &SolItemState) {
        if let SolItemState::Online = state {
            match item {
                SolItem::Drone(drone) => {
                    let val = match drone.get_attrs() {
                        Ok(attrs) => match attrs.get(&ec::attrs::DRONE_BANDWIDTH_USED) {
                            Some(val) => *val,
                            None => OF(0.0),
                        },
                        Err(_) => OF(0.0),
                    };
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
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
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {}
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {}
    pub(in crate::sol::svc) fn effects_started(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        match item {
            SolItem::Module(module) => {
                for effect in effects {
                    if effect.id == ec::effects::ONLINE {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_online.insert(module.get_id());
                    }
                }
            }
            SolItem::Rig(rig) => {
                for effect in effects {
                    if effect.id == ec::effects::RIG_SLOT {
                        if let Some(val) = rig.get_attrs().unwrap().get(&ec::attrs::UPGRADE_COST) {
                            let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                            fit_data.rigs_rigslot_calibration.insert(rig.get_id(), *val);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn effects_stopped(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        match item {
            SolItem::Module(module) => {
                for effect in effects {
                    if effect.id == ec::effects::ONLINE {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_online.remove(&module.get_id());
                    }
                }
            }
            SolItem::Rig(rig) => {
                for effect in effects {
                    if effect.id == ec::effects::RIG_SLOT {
                        let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                        fit_data.rigs_rigslot_calibration.remove(&rig.get_id());
                    }
                }
            }
            _ => (),
        }
    }
}
