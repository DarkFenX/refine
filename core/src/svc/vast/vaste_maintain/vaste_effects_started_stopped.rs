use crate::{
    ad,
    def::{FitKey, ItemKey},
    svc::vast::Vast,
    uad::UadItem,
};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        match item {
            UadItem::Drone(drone) => {
                for a_effect in a_effects {
                    if a_effect.xt.is_active {
                        self.handle_orrs_start(a_effect, item_key, &drone.get_fit_key());
                    }
                }
            }
            UadItem::Fighter(fighter) => {
                for a_effect in a_effects {
                    if a_effect.xt.is_active {
                        self.handle_orrs_start(a_effect, item_key, &fighter.get_fit_key());
                    }
                }
            }
            UadItem::Module(module) => {
                for a_effect in a_effects {
                    if a_effect.xt.is_active {
                        // Local reps
                        if let Some(rep_getter) = a_effect.hc.get_local_shield_rep_opc {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_shield.add_entry(item_key, a_effect.ae.id, rep_getter);
                            if a_effect.hc.charge.is_some() {
                                fit_data
                                    .lr_shield_limitable
                                    .add_entry(item_key, a_effect.ae.id, rep_getter);
                            }
                        }
                        if let Some(rep_getter) = a_effect.hc.get_local_armor_rep_opc {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_armor.add_entry(item_key, a_effect.ae.id, rep_getter);
                            if a_effect.hc.charge.is_some() {
                                fit_data
                                    .lr_armor_limitable
                                    .add_entry(item_key, a_effect.ae.id, rep_getter);
                            }
                        }
                        if let Some(rep_getter) = a_effect.hc.get_local_hull_rep_opc {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_hull.add_entry(item_key, a_effect.ae.id, rep_getter);
                        }
                        // Remote reps
                        self.handle_orrs_start(a_effect, item_key, &module.get_fit_key());
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        match item {
            UadItem::Drone(drone) => {
                for a_effect in a_effects {
                    if a_effect.xt.is_active {
                        self.handle_orrs_stop(a_effect, item_key, &drone.get_fit_key());
                    }
                }
            }
            UadItem::Fighter(fighter) => {
                for a_effect in a_effects {
                    if a_effect.xt.is_active {
                        self.handle_orrs_stop(a_effect, item_key, &fighter.get_fit_key());
                    }
                }
            }
            UadItem::Module(module) => {
                for a_effect in a_effects {
                    if a_effect.xt.is_active {
                        // Local reps
                        if a_effect.hc.get_local_shield_rep_opc.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_shield.remove_l2(&item_key, &a_effect.ae.id);
                            if a_effect.hc.charge.is_some() {
                                fit_data.lr_shield_limitable.remove_l2(&item_key, &a_effect.ae.id);
                            }
                        }
                        if a_effect.hc.get_local_armor_rep_opc.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_armor.remove_l2(&item_key, &a_effect.ae.id);
                            if a_effect.hc.charge.is_some() {
                                fit_data.lr_armor_limitable.remove_l2(&item_key, &a_effect.ae.id);
                            }
                        }
                        if a_effect.hc.get_local_hull_rep_opc.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                            fit_data.lr_hull.remove_l2(&item_key, &a_effect.ae.id);
                        }
                        // Remote reps
                        self.handle_orrs_stop(a_effect, item_key, &module.get_fit_key());
                    }
                }
            }
            _ => (),
        }
    }
    fn handle_orrs_start(&mut self, a_effect: &ad::ArcEffectRt, item_key: ItemKey, fit_key: &FitKey) {
        if let Some(rep_getter) = a_effect.hc.get_remote_shield_rep_opc {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_shield.add_entry(item_key, a_effect.ae.id, rep_getter);
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_armor_rep_opc {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_armor.add_entry(item_key, a_effect.ae.id, rep_getter);
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_hull_rep_opc {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_hull.add_entry(item_key, a_effect.ae.id, rep_getter);
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_cap_rep_opc {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_cap.add_entry(item_key, a_effect.ae.id, rep_getter);
        }
    }
    fn handle_orrs_stop(&mut self, a_effect: &ad::ArcEffectRt, item_key: ItemKey, fit_key: &FitKey) {
        if a_effect.hc.get_remote_shield_rep_opc.is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_shield.remove_l2(&item_key, &a_effect.ae.id);
        }
        if a_effect.hc.get_remote_armor_rep_opc.is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_armor.remove_l2(&item_key, &a_effect.ae.id);
        }
        if a_effect.hc.get_remote_hull_rep_opc.is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_hull.remove_l2(&item_key, &a_effect.ae.id);
        }
        if a_effect.hc.get_remote_cap_rep_opc.is_some() {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_cap.remove_l2(&item_key, &a_effect.ae.id);
        }
    }
}
