use crate::{
    rd,
    svc::vast::Vast,
    ud::{UFitKey, UItem, UItemKey},
};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: UItemKey, item: &UItem, r_effects: &[rd::RcEffect]) {
        match item {
            UItem::Drone(drone) => {
                for r_effect in r_effects {
                    if r_effect.is_active() {
                        self.handle_orrs_start(r_effect, item_key, &drone.get_fit_key());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for r_effect in r_effects {
                    if r_effect.is_active() {
                        self.handle_orrs_start(r_effect, item_key, &fighter.get_fit_key());
                    }
                }
            }
            UItem::Module(module) => {
                for r_effect in r_effects {
                    if r_effect.is_active() {
                        // Damaging effects
                        if let Some(dmg_getter) = r_effect.get_normal_dmg_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.dmg_normal.add_entry(item_key, r_effect.get_id(), dmg_getter);
                        }
                        // Local reps
                        if let Some(rep_getter) = r_effect.get_local_shield_rep_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_shield.add_entry(item_key, r_effect.get_id(), rep_getter);
                            if r_effect.get_charge_info().is_some() {
                                fit_data
                                    .lr_shield_limitable
                                    .add_entry(item_key, r_effect.get_id(), rep_getter);
                            }
                        }
                        if let Some(rep_getter) = r_effect.get_local_armor_rep_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_armor.add_entry(item_key, r_effect.get_id(), rep_getter);
                            if r_effect.get_charge_info().is_some() {
                                fit_data
                                    .lr_armor_limitable
                                    .add_entry(item_key, r_effect.get_id(), rep_getter);
                            }
                        }
                        if let Some(rep_getter) = r_effect.get_local_hull_rep_opc_getter() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_hull.add_entry(item_key, r_effect.get_id(), rep_getter);
                        }
                        // Remote reps
                        self.handle_orrs_start(r_effect, item_key, &module.get_fit_key());
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_key: UItemKey, item: &UItem, r_effects: &[rd::RcEffect]) {
        match item {
            UItem::Drone(drone) => {
                for r_effect in r_effects {
                    if r_effect.is_active() {
                        self.handle_orrs_stop(r_effect, item_key, &drone.get_fit_key());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for r_effect in r_effects {
                    if r_effect.is_active() {
                        self.handle_orrs_stop(r_effect, item_key, &fighter.get_fit_key());
                    }
                }
            }
            UItem::Module(module) => {
                for r_effect in r_effects {
                    if r_effect.is_active() {
                        // Damaging effects
                        if r_effect.get_normal_dmg_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.dmg_normal.remove_l2(&item_key, &r_effect.get_id());
                        }
                        // Local reps
                        if r_effect.get_local_shield_rep_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_shield.remove_l2(&item_key, &r_effect.get_id());
                            if r_effect.get_charge_info().is_some() {
                                fit_data.lr_shield_limitable.remove_l2(&item_key, &r_effect.get_id());
                            }
                        }
                        if r_effect.get_local_armor_rep_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_armor.remove_l2(&item_key, &r_effect.get_id());
                            if r_effect.get_charge_info().is_some() {
                                fit_data.lr_armor_limitable.remove_l2(&item_key, &r_effect.get_id());
                            }
                        }
                        if r_effect.get_local_hull_rep_opc_getter().is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_hull.remove_l2(&item_key, &r_effect.get_id());
                        }
                        // Remote reps
                        self.handle_orrs_stop(r_effect, item_key, &module.get_fit_key());
                    }
                }
            }
            _ => (),
        }
    }
    fn handle_orrs_start(&mut self, r_effect: &rd::RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if let Some(rep_getter) = r_effect.get_remote_shield_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_shield.add_entry(item_key, r_effect.get_id(), rep_getter);
        }
        if let Some(rep_getter) = r_effect.get_remote_armor_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_armor.add_entry(item_key, r_effect.get_id(), rep_getter);
        }
        if let Some(rep_getter) = r_effect.get_remote_hull_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_hull.add_entry(item_key, r_effect.get_id(), rep_getter);
        }
        if let Some(rep_getter) = r_effect.get_remote_cap_rep_opc_getter() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_cap.add_entry(item_key, r_effect.get_id(), rep_getter);
        }
    }
    fn handle_orrs_stop(&mut self, r_effect: &rd::RcEffect, item_key: UItemKey, fit_key: &UFitKey) {
        if r_effect.get_remote_shield_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_shield.remove_l2(&item_key, &r_effect.get_id());
        }
        if r_effect.get_remote_armor_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_armor.remove_l2(&item_key, &r_effect.get_id());
        }
        if r_effect.get_remote_hull_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_hull.remove_l2(&item_key, &r_effect.get_id());
        }
        if r_effect.get_remote_cap_rep_opc_getter().is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_cap.remove_l2(&item_key, &r_effect.get_id());
        }
    }
}
