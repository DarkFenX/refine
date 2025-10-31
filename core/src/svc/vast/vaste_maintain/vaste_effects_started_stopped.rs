use crate::{
    rd::RcEffect,
    svc::vast::Vast,
    ud::{UFitKey, UItem, UItemKey},
};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: UItemKey, item: &UItem, effects: &[RcEffect]) {
        match item {
            UItem::Autocharge(autocharge) => {
                for effect in effects {
                    if effect.is_active() {
                        self.handle_dmg_start(effect, item_key, &autocharge.get_fit_key());
                    }
                }
            }
            UItem::Charge(charge) => {
                for effect in effects {
                    if effect.is_active() {
                        self.handle_dmg_start(effect, item_key, &charge.get_fit_key());
                        self.handle_neut_start(effect, item_key, &charge.get_fit_key());
                    }
                }
            }
            UItem::Drone(drone) => {
                for effect in effects {
                    if effect.is_active_with_duration() {
                        self.handle_dmg_start(effect, item_key, &drone.get_fit_key());
                        self.handle_orrs_start(effect, item_key, &drone.get_fit_key());
                        self.handle_neut_start(effect, item_key, &drone.get_fit_key());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for effect in effects {
                    if effect.is_active_with_duration() {
                        self.handle_dmg_start(effect, item_key, &fighter.get_fit_key());
                        self.handle_orrs_start(effect, item_key, &fighter.get_fit_key());
                        self.handle_neut_start(effect, item_key, &fighter.get_fit_key());
                    }
                }
            }
            UItem::Module(module) => {
                for effect in effects {
                    if effect.is_active_with_duration() {
                        self.handle_dmg_start(effect, item_key, &module.get_fit_key());
                        // Local reps
                        if let Some(rep_getter) = effect.get_local_shield_rep_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_shield.add_entry(item_key, effect.get_key(), rep_getter);
                            if effect.get_charge_info().is_some() {
                                fit_data
                                    .lr_shield_limitable
                                    .add_entry(item_key, effect.get_key(), rep_getter);
                            }
                        }
                        if let Some(rep_getter) = effect.get_local_armor_rep_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_armor.add_entry(item_key, effect.get_key(), rep_getter);
                            if effect.get_charge_info().is_some() {
                                fit_data
                                    .lr_armor_limitable
                                    .add_entry(item_key, effect.get_key(), rep_getter);
                            }
                        }
                        if let Some(rep_getter) = effect.get_local_hull_rep_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_hull.add_entry(item_key, effect.get_key(), rep_getter);
                        }
                        // Remote reps
                        self.handle_orrs_start(effect, item_key, &module.get_fit_key());
                        // Cap
                        if let Some(cap_boost_getter) = effect.get_cap_boost_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data
                                .cap_boosts
                                .add_entry(item_key, effect.get_key(), cap_boost_getter);
                        }
                        self.handle_neut_start(effect, item_key, &module.get_fit_key());
                        if effect.get_discharge_attr_id().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.cap_users.add_entry(item_key, effect.get_key());
                        }
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_key: UItemKey, item: &UItem, effects: &[RcEffect]) {
        match item {
            UItem::Autocharge(autocharge) => {
                for effect in effects {
                    if effect.is_active() {
                        self.handle_dmg_stop(effect, item_key, &autocharge.get_fit_key());
                    }
                }
            }
            UItem::Charge(charge) => {
                for effect in effects {
                    if effect.is_active() {
                        self.handle_dmg_stop(effect, item_key, &charge.get_fit_key());
                        self.handle_neut_stop(effect, item_key, &charge.get_fit_key());
                    }
                }
            }
            UItem::Drone(drone) => {
                for effect in effects {
                    if effect.is_active_with_duration() {
                        self.handle_dmg_stop(effect, item_key, &drone.get_fit_key());
                        self.handle_orrs_stop(effect, item_key, &drone.get_fit_key());
                        self.handle_neut_stop(effect, item_key, &drone.get_fit_key());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for effect in effects {
                    if effect.is_active_with_duration() {
                        self.handle_dmg_stop(effect, item_key, &fighter.get_fit_key());
                        self.handle_orrs_stop(effect, item_key, &fighter.get_fit_key());
                        self.handle_neut_stop(effect, item_key, &fighter.get_fit_key());
                    }
                }
            }
            UItem::Module(module) => {
                for effect in effects {
                    if effect.is_active_with_duration() {
                        self.handle_dmg_stop(effect, item_key, &module.get_fit_key());
                        // Local reps
                        if effect.get_local_shield_rep_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_shield.remove_l2(&item_key, &effect.get_key());
                            if effect.get_charge_info().is_some() {
                                fit_data.lr_shield_limitable.remove_l2(&item_key, &effect.get_key());
                            }
                        }
                        if effect.get_local_armor_rep_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_armor.remove_l2(&item_key, &effect.get_key());
                            if effect.get_charge_info().is_some() {
                                fit_data.lr_armor_limitable.remove_l2(&item_key, &effect.get_key());
                            }
                        }
                        if effect.get_local_hull_rep_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_hull.remove_l2(&item_key, &effect.get_key());
                        }
                        // Remote reps
                        self.handle_orrs_stop(effect, item_key, &module.get_fit_key());
                        // Cap
                        if effect.get_cap_boost_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.cap_boosts.remove_l2(&item_key, &effect.get_key());
                        }
                        self.handle_neut_stop(effect, item_key, &module.get_fit_key());
                        if effect.get_discharge_attr_id().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.cap_users.remove_entry(&item_key, &effect.get_key());
                        }
                    }
                }
            }
            _ => (),
        }
    }
    fn handle_dmg_start(&mut self, effect: &RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if let Some(dmg_getter) = effect.get_normal_dmg_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.dmg_normal.add_entry(item_key, effect.get_key(), dmg_getter);
        }
        if let Some(dmg_getter) = effect.get_breacher_dmg_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.dmg_breacher.add_entry(item_key, effect.get_key(), dmg_getter);
        }
    }
    fn handle_dmg_stop(&mut self, effect: &RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if effect.get_normal_dmg_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.dmg_normal.remove_l2(&item_key, &effect.get_key());
        }
        if effect.get_breacher_dmg_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.dmg_breacher.remove_l2(&item_key, &effect.get_key());
        }
    }
    fn handle_orrs_start(&mut self, effect: &RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if let Some(rep_getter) = effect.get_remote_shield_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_shield.add_entry(item_key, effect.get_key(), rep_getter);
        }
        if let Some(rep_getter) = effect.get_remote_armor_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_armor.add_entry(item_key, effect.get_key(), rep_getter);
        }
        if let Some(rep_getter) = effect.get_remote_hull_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_hull.add_entry(item_key, effect.get_key(), rep_getter);
        }
        if let Some(rep_getter) = effect.get_remote_cap_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.out_cap.add_entry(item_key, effect.get_key(), rep_getter);
        }
    }
    fn handle_orrs_stop(&mut self, effect: &RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if effect.get_remote_shield_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_shield.remove_l2(&item_key, &effect.get_key());
        }
        if effect.get_remote_armor_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_armor.remove_l2(&item_key, &effect.get_key());
        }
        if effect.get_remote_hull_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_hull.remove_l2(&item_key, &effect.get_key());
        }
        if effect.get_remote_cap_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.out_cap.remove_l2(&item_key, &effect.get_key());
        }
    }
    fn handle_neut_start(&mut self, effect: &RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if let Some(neut_getter) = effect.get_neut_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.out_neuts.add_entry(item_key, effect.get_key(), neut_getter);
        }
    }
    fn handle_neut_stop(&mut self, effect: &RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if effect.get_neut_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.out_neuts.remove_l2(&item_key, &effect.get_key());
        }
    }
}
