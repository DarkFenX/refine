use crate::{
    misc::EffectSpec,
    rd::{REffect, RcEffect},
    svc::vast::{Vast, validators::EffectSecZoneInfo},
    ud::{UFitId, UItem, UItemId},
};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_uid: UItemId, item: &UItem, effects: &[RcEffect]) {
        match item {
            UItem::Autocharge(autocharge) => {
                for effect in effects {
                    self.handle_aggr_start(effect, item_uid, &autocharge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_start(effect, item_uid, &autocharge.get_fit_uid());
                    }
                }
            }
            UItem::Charge(charge) => {
                for effect in effects {
                    self.handle_aggr_start(effect, item_uid, &charge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_start(effect, item_uid, &charge.get_fit_uid());
                        self.handle_neut_start(effect, item_uid, &charge.get_fit_uid());
                    }
                }
            }
            UItem::Drone(drone) => {
                for effect in effects {
                    self.handle_aggr_start(effect, item_uid, &drone.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_start(effect, item_uid, &drone.get_fit_uid());
                        self.handle_mining_start(effect, item_uid, &drone.get_fit_uid());
                        self.handle_orrs_start(effect, item_uid, &drone.get_fit_uid());
                        self.handle_neut_start(effect, item_uid, &drone.get_fit_uid());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for effect in effects {
                    self.handle_aggr_start(effect, item_uid, &fighter.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_start(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_orrs_start(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_neut_start(effect, item_uid, &fighter.get_fit_uid());
                    }
                    if effect.banned_in_hisec || effect.banned_in_lowsec {
                        let fit_data = self.get_fit_data_mut(&fighter.get_fit_uid());
                        fit_data.sec_zone_effect.add_entry(
                            item_uid,
                            effect.rid,
                            EffectSecZoneInfo {
                                banned_in_hisec: effect.banned_in_hisec,
                                banned_in_lowsec: effect.banned_in_lowsec,
                            },
                        )
                    }
                }
            }
            UItem::Module(module) => {
                for effect in effects {
                    self.handle_aggr_start(effect, item_uid, &module.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_start(effect, item_uid, &module.get_fit_uid());
                        self.handle_mining_start(effect, item_uid, &module.get_fit_uid());
                        // Local reps
                        if let Some(rep_ospec) = effect.local_shield_rep_opc_spec {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_shield.add_entry(item_uid, effect.rid, rep_ospec);
                            if effect.charge.is_some() {
                                fit_data.lr_shield_limitable.add_entry(item_uid, effect.rid, rep_ospec);
                            }
                        }
                        if let Some(rep_ospec) = effect.local_armor_rep_opc_spec {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_armor.add_entry(item_uid, effect.rid, rep_ospec);
                            if effect.charge.is_some() {
                                fit_data.lr_armor_limitable.add_entry(item_uid, effect.rid, rep_ospec);
                            }
                        }
                        if let Some(rep_ospec) = effect.local_hull_rep_opc_spec {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_hull.add_entry(item_uid, effect.rid, rep_ospec);
                        }
                        // Outgoing reps
                        self.handle_orrs_start(effect, item_uid, &module.get_fit_uid());
                        // Cap
                        if let Some(inject_ospec) = effect.cap_inject_opc_spec {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_injects.add_entry(item_uid, effect.rid, inject_ospec);
                        }
                        self.handle_neut_start(effect, item_uid, &module.get_fit_uid());
                        if let Some(use_attr_rid) = effect.discharge_attr_rid {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data
                                .cap_consumers_active
                                .add_entry(item_uid, effect.rid, use_attr_rid);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_uid: UItemId, item: &UItem, effects: &[RcEffect]) {
        match item {
            UItem::Autocharge(autocharge) => {
                for effect in effects {
                    self.handle_aggr_stop(effect, item_uid, &autocharge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_stop(effect, item_uid, &autocharge.get_fit_uid());
                    }
                }
            }
            UItem::Charge(charge) => {
                for effect in effects {
                    self.handle_aggr_stop(effect, item_uid, &charge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_stop(effect, item_uid, &charge.get_fit_uid());
                        self.handle_neut_stop(effect, item_uid, &charge.get_fit_uid());
                    }
                }
            }
            UItem::Drone(drone) => {
                for effect in effects {
                    self.handle_aggr_stop(effect, item_uid, &drone.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_stop(effect, item_uid, &drone.get_fit_uid());
                        self.handle_mining_stop(effect, item_uid, &drone.get_fit_uid());
                        self.handle_orrs_stop(effect, item_uid, &drone.get_fit_uid());
                        self.handle_neut_stop(effect, item_uid, &drone.get_fit_uid());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for effect in effects {
                    self.handle_aggr_stop(effect, item_uid, &fighter.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_stop(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_orrs_stop(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_neut_stop(effect, item_uid, &fighter.get_fit_uid());
                    }
                    for effect in effects {
                        if effect.banned_in_hisec || effect.banned_in_lowsec {
                            let fit_data = self.get_fit_data_mut(&fighter.get_fit_uid());
                            fit_data.sec_zone_effect.remove_l2(item_uid, &effect.rid);
                        }
                    }
                }
            }
            UItem::Module(module) => {
                for effect in effects {
                    self.handle_aggr_stop(effect, item_uid, &module.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_stop(effect, item_uid, &module.get_fit_uid());
                        self.handle_mining_stop(effect, item_uid, &module.get_fit_uid());
                        // Local reps
                        if effect.local_shield_rep_opc_spec.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_shield.remove_l2(item_uid, &effect.rid);
                            if effect.charge.is_some() {
                                fit_data.lr_shield_limitable.remove_l2(item_uid, &effect.rid);
                            }
                        }
                        if effect.local_armor_rep_opc_spec.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_armor.remove_l2(item_uid, &effect.rid);
                            if effect.charge.is_some() {
                                fit_data.lr_armor_limitable.remove_l2(item_uid, &effect.rid);
                            }
                        }
                        if effect.local_hull_rep_opc_spec.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_hull.remove_l2(item_uid, &effect.rid);
                        }
                        // Outgoing reps
                        self.handle_orrs_stop(effect, item_uid, &module.get_fit_uid());
                        // Cap
                        if effect.cap_inject_opc_spec.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_injects.remove_l2(item_uid, &effect.rid);
                        }
                        self.handle_neut_stop(effect, item_uid, &module.get_fit_uid());
                        if effect.discharge_attr_rid.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_consumers_active.remove_l2(item_uid, &effect.rid);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    fn handle_aggr_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.is_offense {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.aggro_effects.insert(EffectSpec::new(item_uid, effect.rid));
        }
    }
    fn handle_aggr_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.is_offense {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.aggro_effects.remove(&EffectSpec::new(item_uid, effect.rid));
        }
    }
    fn handle_dmg_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if let Some(dmg_ospec) = effect.normal_dmg_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_normal.add_entry(item_uid, effect.rid, dmg_ospec);
        }
        if let Some(dmg_getter) = effect.breacher_dmg_opc_getter {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_breacher.add_entry(item_uid, effect.rid, dmg_getter);
        }
    }
    fn handle_dmg_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.normal_dmg_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_normal.remove_l2(item_uid, &effect.rid);
        }
        if effect.breacher_dmg_opc_getter.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_breacher.remove_l2(item_uid, &effect.rid);
        }
    }
    fn handle_mining_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if let Some(mining_ospec) = effect.mining_ore_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ore.add_entry(item_uid, effect.rid, mining_ospec);
        }
        if let Some(mining_ospec) = effect.mining_ice_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ice.add_entry(item_uid, effect.rid, mining_ospec);
        }
        if let Some(mining_ospec) = effect.mining_gas_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_gas.add_entry(item_uid, effect.rid, mining_ospec);
        }
    }
    fn handle_mining_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.mining_ore_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ore.remove_l2(item_uid, &effect.rid);
        }
        if effect.mining_ice_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ice.remove_l2(item_uid, &effect.rid);
        }
        if effect.mining_gas_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_gas.remove_l2(item_uid, &effect.rid);
        }
    }
    fn handle_orrs_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if let Some(rep_ospec) = effect.outgoing_shield_rep_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_shield.add_entry(item_uid, effect.rid, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_armor_rep_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_armor.add_entry(item_uid, effect.rid, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_hull_rep_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_hull.add_entry(item_uid, effect.rid, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_cap_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_cap.add_entry(item_uid, effect.rid, rep_ospec);
        }
    }
    fn handle_orrs_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.outgoing_shield_rep_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_shield.remove_l2(item_uid, &effect.rid);
        }
        if effect.outgoing_armor_rep_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_armor.remove_l2(item_uid, &effect.rid);
        }
        if effect.outgoing_hull_rep_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_hull.remove_l2(item_uid, &effect.rid);
        }
        if effect.outgoing_cap_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_cap.remove_l2(item_uid, &effect.rid);
        }
    }
    fn handle_neut_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if let Some(neut_ospec) = effect.neut_opc_spec {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_neuts.add_entry(item_uid, effect.rid, neut_ospec);
        }
    }
    fn handle_neut_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.neut_opc_spec.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_neuts.remove_l2(item_uid, &effect.rid);
        }
    }
}
