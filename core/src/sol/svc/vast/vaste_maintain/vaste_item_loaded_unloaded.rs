use crate::{
    ad, ec,
    sol::{svc::vast::SolVast, uad::item::SolItem},
};

impl SolVast {
    pub(in crate::sol::svc) fn item_loaded(&mut self, item: &SolItem) {
        match item {
            SolItem::Module(module) => {
                let extras = module.get_a_extras().unwrap();
                if let Some(ship_limit) = &extras.ship_limit {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    fit_data
                        .ship_limited_mods_rigs_subs
                        .insert(module.get_id(), ship_limit.clone());
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    fit_data
                        .mods_rigs_max_group_fitted_all
                        .add_entry(grp_id, module.get_id());
                    if module.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_FITTED) {
                        fit_data.mods_rigs_max_group_fitted_limited.insert(module.get_id());
                    }
                }
            }
            SolItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                if let Some(ship_limit) = &extras.ship_limit {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                    fit_data
                        .ship_limited_mods_rigs_subs
                        .insert(rig.get_id(), ship_limit.clone());
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                    fit_data.mods_rigs_max_group_fitted_all.add_entry(grp_id, rig.get_id());
                    if rig.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_FITTED) {
                        fit_data.mods_rigs_max_group_fitted_limited.insert(rig.get_id());
                    }
                }
            }
            SolItem::Drone(drone) => {
                if let Some(val) = drone.get_a_extras().unwrap().volume {
                    let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
                    fit_data.drones_volume.insert(drone.get_id(), val);
                }
            }
            SolItem::Implant(implant) => {
                if let Some(ad::AItemKind::Implant(slot)) = implant.get_a_extras().unwrap().kind {
                    let fit_data = self.get_fit_data_mut(&implant.get_fit_id()).unwrap();
                    fit_data.slotted_implants.add_entry(slot, implant.get_id());
                }
            }
            SolItem::Booster(booster) => {
                if let Some(ad::AItemKind::Booster(slot)) = booster.get_a_extras().unwrap().kind {
                    let fit_data = self.get_fit_data_mut(&booster.get_fit_id()).unwrap();
                    fit_data.slotted_boosters.add_entry(slot, booster.get_id());
                }
            }
            SolItem::Subsystem(subsystem) => {
                let fit_data = self.get_fit_data_mut(&subsystem.get_fit_id()).unwrap();
                let extras = subsystem.get_a_extras().unwrap();
                if let Some(ad::AItemKind::Subsystem(slot)) = extras.kind {
                    fit_data.slotted_subsystems.add_entry(slot, subsystem.get_id());
                }
                if let Some(ship_limit) = &extras.ship_limit {
                    fit_data
                        .ship_limited_mods_rigs_subs
                        .insert(subsystem.get_id(), ship_limit.clone());
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_unloaded(&mut self, item: &SolItem) {
        match item {
            SolItem::Module(module) => {
                let extras = module.get_a_extras().unwrap();
                if extras.ship_limit.is_some() {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    fit_data.ship_limited_mods_rigs_subs.remove(&module.get_id());
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    fit_data
                        .mods_rigs_max_group_fitted_all
                        .remove_entry(&grp_id, &module.get_id());
                    fit_data.mods_rigs_max_group_fitted_limited.remove(&module.get_id());
                }
            }
            SolItem::Rig(rig) => {
                let extras = rig.get_a_extras().unwrap();
                if extras.ship_limit.is_some() {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                    fit_data.ship_limited_mods_rigs_subs.remove(&rig.get_id());
                }
                if let Some(grp_id) = extras.val_fitted_group_id {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                    fit_data
                        .mods_rigs_max_group_fitted_all
                        .remove_entry(&grp_id, &rig.get_id());
                    fit_data.mods_rigs_max_group_fitted_limited.remove(&rig.get_id());
                }
            }
            SolItem::Drone(drone) => {
                let fit_data = self.get_fit_data_mut(&drone.get_fit_id()).unwrap();
                fit_data.drones_volume.remove(&drone.get_id());
            }
            SolItem::Implant(implant) => {
                if let Some(ad::AItemKind::Implant(slot)) = implant.get_a_extras().unwrap().kind {
                    let fit_data = self.get_fit_data_mut(&implant.get_fit_id()).unwrap();
                    fit_data.slotted_implants.remove_entry(&slot, &implant.get_id());
                }
            }
            SolItem::Booster(booster) => {
                if let Some(ad::AItemKind::Booster(slot)) = booster.get_a_extras().unwrap().kind {
                    let fit_data = self.get_fit_data_mut(&booster.get_fit_id()).unwrap();
                    fit_data.slotted_boosters.remove_entry(&slot, &booster.get_id());
                }
            }
            SolItem::Subsystem(subsystem) => {
                let fit_data = self.get_fit_data_mut(&subsystem.get_fit_id()).unwrap();
                let extras = subsystem.get_a_extras().unwrap();
                if let Some(ad::AItemKind::Subsystem(slot)) = extras.kind {
                    fit_data.slotted_subsystems.remove_entry(&slot, &subsystem.get_id());
                }
                if extras.ship_limit.is_some() {
                    fit_data.ship_limited_mods_rigs_subs.remove(&subsystem.get_id());
                }
            }
            _ => (),
        }
    }
}
