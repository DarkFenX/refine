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
                        Some(attrs) => match attrs.get(&ec::attrs::DRONE_BANDWIDTH_USED) {
                            Some(val) => *val,
                            None => OF(0.0),
                        },
                        None => OF(0.0),
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
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if let SolItemState::Online = state {
            if let SolItem::Fighter(fighter) = item {
                if let Some(ad::AItemKind::FighterSquad(fighter_kind)) = fighter.get_a_item_kind().unwrap() {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    match fighter_kind {
                        ad::AFighterKind::Support => fit_data.support_fighters_online.insert(fighter.get_id()),
                        ad::AFighterKind::Light => fit_data.light_fighters_online.insert(fighter.get_id()),
                        ad::AFighterKind::Heavy => fit_data.heavy_fighters_online.insert(fighter.get_id()),
                        ad::AFighterKind::StandupSupport => {
                            fit_data.standup_support_fighters_online.insert(fighter.get_id())
                        }
                        ad::AFighterKind::StandupLight => {
                            fit_data.standup_light_fighters_online.insert(fighter.get_id())
                        }
                        ad::AFighterKind::StandupHeavy => {
                            fit_data.standup_heavy_fighters_online.insert(fighter.get_id())
                        }
                    };
                }
            }
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if let SolItemState::Online = state {
            if let SolItem::Fighter(fighter) = item {
                if let Some(ad::AItemKind::FighterSquad(fighter_kind)) = fighter.get_a_item_kind().unwrap() {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    match fighter_kind {
                        ad::AFighterKind::Support => fit_data.support_fighters_online.remove(&fighter.get_id()),
                        ad::AFighterKind::Light => fit_data.light_fighters_online.remove(&fighter.get_id()),
                        ad::AFighterKind::Heavy => fit_data.heavy_fighters_online.remove(&fighter.get_id()),
                        ad::AFighterKind::StandupSupport => {
                            fit_data.standup_support_fighters_online.remove(&fighter.get_id())
                        }
                        ad::AFighterKind::StandupLight => {
                            fit_data.standup_light_fighters_online.remove(&fighter.get_id())
                        }
                        ad::AFighterKind::StandupHeavy => {
                            fit_data.standup_heavy_fighters_online.remove(&fighter.get_id())
                        }
                    };
                }
                let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                fit_data.support_fighters_online.remove(&fighter.get_id());
                fit_data.light_fighters_online.remove(&fighter.get_id());
                fit_data.heavy_fighters_online.remove(&fighter.get_id());
                fit_data.standup_support_fighters_online.remove(&fighter.get_id());
                fit_data.standup_light_fighters_online.remove(&fighter.get_id());
                fit_data.standup_heavy_fighters_online.remove(&fighter.get_id());
            }
        }
    }
    pub(in crate::sol::svc) fn effects_started(&mut self, item: &SolItem, effects: &[ad::ArcEffect]) {
        match item {
            SolItem::Module(module) => {
                for effect in effects {
                    match effect.id {
                        ec::effects::ONLINE => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_online.insert(module.get_id());
                        }
                        ec::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.insert(module.get_id());
                        }
                        ec::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.insert(module.get_id());
                        }
                        _ => (),
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
    pub(in crate::sol::svc) fn effects_stopped(&mut self, item: &SolItem, effects: &[ad::ArcEffect]) {
        match item {
            SolItem::Module(module) => {
                for effect in effects {
                    match effect.id {
                        ec::effects::ONLINE => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_online.remove(&module.get_id());
                        }
                        ec::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.remove(&module.get_id());
                        }
                        ec::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.remove(&module.get_id());
                        }
                        _ => (),
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
