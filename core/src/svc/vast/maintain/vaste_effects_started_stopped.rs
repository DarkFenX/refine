use crate::{
    misc::EffectSpec,
    rd::{RAttrConsts, REffect, RcEffect},
    svc::vast::{Vast, validators::EffectSecZoneInfo},
    ud::{UFitId, UItem, UItemId},
};

impl Vast {
    pub(in crate::svc) fn effects_started(
        &mut self,
        attr_consts: &RAttrConsts,
        item_uid: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        match item {
            UItem::Autocharge(autocharge) => {
                for effect in effects {
                    self.handle_aggro_start(effect, item_uid, &autocharge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_start(effect, item_uid, &autocharge.get_fit_uid());
                    }
                }
            }
            UItem::Charge(charge) => {
                for effect in effects {
                    self.handle_aggro_start(effect, item_uid, &charge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_start(effect, item_uid, &charge.get_fit_uid());
                        self.handle_neut_start(attr_consts, effect, item_uid, item, &charge.get_fit_uid());
                    }
                }
            }
            UItem::Drone(drone) => {
                for effect in effects {
                    self.handle_aggro_start(effect, item_uid, &drone.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_start(effect, item_uid, &drone.get_fit_uid());
                        self.handle_mining_start(effect, item_uid, item, &drone.get_fit_uid());
                        self.handle_orrs_start(effect, item_uid, &drone.get_fit_uid());
                        self.handle_neut_start(attr_consts, effect, item_uid, item, &drone.get_fit_uid());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for effect in effects {
                    self.handle_aggro_start(effect, item_uid, &fighter.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_start(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_orrs_start(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_neut_start(attr_consts, effect, item_uid, item, &fighter.get_fit_uid());
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
                    self.handle_aggro_start(effect, item_uid, &module.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_start(effect, item_uid, &module.get_fit_uid());
                        self.handle_mining_start(effect, item_uid, item, &module.get_fit_uid());
                        // Local reps
                        if let Some(rep_ospec) = effect.local_shield_rep {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_shield.add_entry(item_uid, effect.rid, rep_ospec);
                            if effect.charge.is_some() {
                                fit_data.lr_shield_limitable.add_entry(item_uid, effect.rid, rep_ospec);
                            }
                        }
                        if let Some(rep_ospec) = effect.local_armor_rep {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_armor.add_entry(item_uid, effect.rid, rep_ospec);
                            if effect.charge.is_some() {
                                fit_data.lr_armor_limitable.add_entry(item_uid, effect.rid, rep_ospec);
                            }
                        }
                        if let Some(rep_ospec) = effect.local_hull_rep {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_hull.add_entry(item_uid, effect.rid, rep_ospec);
                        }
                        // Outgoing reps
                        self.handle_orrs_start(effect, item_uid, &module.get_fit_uid());
                        // Cap
                        if let Some(cap_ospec) = effect.cap_consume {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_consumers.add_entry(item_uid, effect.rid, cap_ospec);
                        }
                        if let Some(nosf_ospec) = effect.nosf {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_nosfs.add_entry(item_uid, effect.rid, nosf_ospec);
                        }
                        if let Some(inject_ospec) = effect.cap_inject {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_injects.add_entry(item_uid, effect.rid, inject_ospec);
                        }
                        self.handle_neut_start(attr_consts, effect, item_uid, item, &module.get_fit_uid());
                    }
                    if effect.cloaks_carrier {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                        fit_data
                            .mods_active_cloaks
                            .insert(EffectSpec::new(item_uid, effect.rid));
                    }
                    if effect.disallows_cloak {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                        fit_data
                            .effects_disallow_cloak
                            .insert(EffectSpec::new(item_uid, effect.rid));
                    }
                    if effect.disallows_jump_wh {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                        fit_data
                            .effects_disallow_jump_wh
                            .insert(EffectSpec::new(item_uid, effect.rid));
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn effects_stopped(
        &mut self,
        attr_consts: &RAttrConsts,
        item_uid: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        match item {
            UItem::Autocharge(autocharge) => {
                for effect in effects {
                    self.handle_aggro_stop(effect, item_uid, &autocharge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_stop(effect, item_uid, &autocharge.get_fit_uid());
                    }
                }
            }
            UItem::Charge(charge) => {
                for effect in effects {
                    self.handle_aggro_stop(effect, item_uid, &charge.get_fit_uid());
                    if effect.is_active() {
                        self.handle_dmg_stop(effect, item_uid, &charge.get_fit_uid());
                        self.handle_neut_stop(attr_consts, effect, item_uid, item, &charge.get_fit_uid());
                    }
                }
            }
            UItem::Drone(drone) => {
                for effect in effects {
                    self.handle_aggro_stop(effect, item_uid, &drone.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_stop(effect, item_uid, &drone.get_fit_uid());
                        self.handle_mining_stop(effect, item_uid, item, &drone.get_fit_uid());
                        self.handle_orrs_stop(effect, item_uid, &drone.get_fit_uid());
                        self.handle_neut_stop(attr_consts, effect, item_uid, item, &drone.get_fit_uid());
                    }
                }
            }
            UItem::Fighter(fighter) => {
                for effect in effects {
                    self.handle_aggro_stop(effect, item_uid, &fighter.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_stop(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_orrs_stop(effect, item_uid, &fighter.get_fit_uid());
                        self.handle_neut_stop(attr_consts, effect, item_uid, item, &fighter.get_fit_uid());
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
                    self.handle_aggro_stop(effect, item_uid, &module.get_fit_uid());
                    if effect.is_active_with_duration {
                        self.handle_dmg_stop(effect, item_uid, &module.get_fit_uid());
                        self.handle_mining_stop(effect, item_uid, item, &module.get_fit_uid());
                        // Local reps
                        if effect.local_shield_rep.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_shield.remove_l2(item_uid, &effect.rid);
                            if effect.charge.is_some() {
                                fit_data.lr_shield_limitable.remove_l2(item_uid, &effect.rid);
                            }
                        }
                        if effect.local_armor_rep.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_armor.remove_l2(item_uid, &effect.rid);
                            if effect.charge.is_some() {
                                fit_data.lr_armor_limitable.remove_l2(item_uid, &effect.rid);
                            }
                        }
                        if effect.local_hull_rep.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.lr_hull.remove_l2(item_uid, &effect.rid);
                        }
                        // Outgoing reps
                        self.handle_orrs_stop(effect, item_uid, &module.get_fit_uid());
                        // Cap
                        if effect.cap_consume.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_consumers.remove_l2(item_uid, &effect.rid);
                        }
                        if effect.nosf.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_nosfs.remove_l2(item_uid, &effect.rid);
                        }
                        if effect.cap_inject.is_some() {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                            fit_data.cap_injects.remove_l2(item_uid, &effect.rid);
                        }
                        self.handle_neut_stop(attr_consts, effect, item_uid, item, &module.get_fit_uid());
                    }
                    if effect.cloaks_carrier {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                        fit_data
                            .mods_active_cloaks
                            .remove(&EffectSpec::new(item_uid, effect.rid));
                    }
                    if effect.disallows_cloak {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                        fit_data
                            .effects_disallow_cloak
                            .remove(&EffectSpec::new(item_uid, effect.rid));
                    }
                    if effect.disallows_jump_wh {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                        fit_data
                            .effects_disallow_jump_wh
                            .remove(&EffectSpec::new(item_uid, effect.rid));
                    }
                }
            }
            _ => (),
        }
    }
    fn handle_aggro_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.aggro {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.effects_aggro.insert(EffectSpec::new(item_uid, effect.rid));
        }
    }
    fn handle_aggro_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.aggro {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.effects_aggro.remove(&EffectSpec::new(item_uid, effect.rid));
        }
    }
    fn handle_dmg_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if let Some(dmg_ospec) = effect.normal_dmg {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_normal.add_entry(item_uid, effect.rid, dmg_ospec);
        }
        if let Some(dmg_ospec) = effect.breacher_dmg {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_breacher.add_entry(item_uid, effect.rid, dmg_ospec);
        }
    }
    fn handle_dmg_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.normal_dmg.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_normal.remove_l2(item_uid, &effect.rid);
        }
        if effect.breacher_dmg.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.dmg_breacher.remove_l2(item_uid, &effect.rid);
        }
    }
    fn handle_mining_start(&mut self, effect: &REffect, item_uid: UItemId, item: &UItem, fit_uid: &UFitId) {
        if let Some(mining) = &effect.mining_ore
            && mining.check(item)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ore.add_entry(item_uid, effect.rid, mining.ospec);
        }
        if let Some(mining) = &effect.mining_ice
            && mining.check(item)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ice.add_entry(item_uid, effect.rid, mining.ospec);
        }
        if let Some(mining) = &effect.mining_gas
            && mining.check(item)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_gas.add_entry(item_uid, effect.rid, mining.ospec);
        }
    }
    fn handle_mining_stop(&mut self, effect: &REffect, item_uid: UItemId, item: &UItem, fit_uid: &UFitId) {
        if let Some(mining) = &effect.mining_ore
            && mining.check(item)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ore.remove_l2(item_uid, &effect.rid);
        }
        if let Some(mining) = &effect.mining_ice
            && mining.check(item)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_ice.remove_l2(item_uid, &effect.rid);
        }
        if let Some(mining) = &effect.mining_gas
            && mining.check(item)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.mining_gas.remove_l2(item_uid, &effect.rid);
        }
    }
    fn handle_orrs_start(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if let Some(rep_ospec) = effect.outgoing_shield_rep {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_shield.add_entry(item_uid, effect.rid, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_armor_rep {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_armor.add_entry(item_uid, effect.rid, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_hull_rep {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_hull.add_entry(item_uid, effect.rid, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_cap {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_cap.add_entry(item_uid, effect.rid, rep_ospec);
        }
    }
    fn handle_orrs_stop(&mut self, effect: &REffect, item_uid: UItemId, fit_uid: &UFitId) {
        if effect.outgoing_shield_rep.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_shield.remove_l2(item_uid, &effect.rid);
        }
        if effect.outgoing_armor_rep.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_armor.remove_l2(item_uid, &effect.rid);
        }
        if effect.outgoing_hull_rep.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.orr_hull.remove_l2(item_uid, &effect.rid);
        }
        if effect.outgoing_cap.is_some() {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_cap.remove_l2(item_uid, &effect.rid);
        }
    }
    fn handle_neut_start(
        &mut self,
        attr_consts: &RAttrConsts,
        effect: &REffect,
        item_uid: UItemId,
        item: &UItem,
        fit_uid: &UFitId,
    ) {
        if let Some(neut) = &effect.neut
            && neut.check(item, attr_consts)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_neuts.add_entry(item_uid, effect.rid, neut.ospec);
        }
    }
    fn handle_neut_stop(
        &mut self,
        attr_consts: &RAttrConsts,
        effect: &REffect,
        item_uid: UItemId,
        item: &UItem,
        fit_uid: &UFitId,
    ) {
        if let Some(neut) = &effect.neut
            && neut.check(item, attr_consts)
        {
            let fit_data = self.get_fit_data_mut(fit_uid);
            fit_data.out_neuts.remove_l2(item_uid, &effect.rid);
        }
    }
}
