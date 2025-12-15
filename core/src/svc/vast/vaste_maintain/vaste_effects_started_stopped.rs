use super::cache::FDCache;
use crate::{
    misc::EffectSpec,
    rd::RcEffect,
    svc::vast::{Vast, vaste_vals::EffectSecZoneInfo},
    ud::{UItem, UItemKey},
};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: UItemKey, item: &UItem, effects: &[RcEffect]) {
        match item {
            UItem::Autocharge(autocharge) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, autocharge.get_fit_key());
                for effect in effects {
                    handle_aggr_start(&mut fd_cache, effect, item_key);
                    if effect.is_active() {
                        handle_dmg_start(&mut fd_cache, effect, item_key);
                    }
                }
            }
            UItem::Charge(charge) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, charge.get_fit_key());
                for effect in effects {
                    handle_aggr_start(&mut fd_cache, effect, item_key);
                    if effect.is_active() {
                        handle_dmg_start(&mut fd_cache, effect, item_key);
                        handle_neut_start(&mut fd_cache, effect, item_key);
                    }
                }
            }
            UItem::Drone(drone) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, drone.get_fit_key());
                for effect in effects {
                    handle_aggr_start(&mut fd_cache, effect, item_key);
                    if effect.is_active_with_duration {
                        handle_dmg_start(&mut fd_cache, effect, item_key);
                        handle_mining_start(&mut fd_cache, effect, item_key);
                        handle_orrs_start(&mut fd_cache, effect, item_key);
                        handle_neut_start(&mut fd_cache, effect, item_key);
                    }
                }
            }
            UItem::Fighter(fighter) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, fighter.get_fit_key());
                for effect in effects {
                    handle_aggr_start(&mut fd_cache, effect, item_key);
                    if effect.is_active_with_duration {
                        handle_dmg_start(&mut fd_cache, effect, item_key);
                        handle_orrs_start(&mut fd_cache, effect, item_key);
                        handle_neut_start(&mut fd_cache, effect, item_key);
                    }
                    if effect.banned_in_hisec || effect.banned_in_lowsec {
                        let fit_data = fd_cache.get();
                        fit_data.sec_zone_effect.add_entry(
                            item_key,
                            effect.key,
                            EffectSecZoneInfo {
                                banned_in_hisec: effect.banned_in_hisec,
                                banned_in_lowsec: effect.banned_in_lowsec,
                            },
                        )
                    }
                }
            }
            UItem::Module(module) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, module.get_fit_key());
                for effect in effects {
                    handle_aggr_start(&mut fd_cache, effect, item_key);
                    if effect.is_active_with_duration {
                        handle_dmg_start(&mut fd_cache, effect, item_key);
                        handle_mining_start(&mut fd_cache, effect, item_key);
                        // Local reps
                        if let Some(rep_getter) = effect.local_shield_rep_opc_getter {
                            let fit_data = fd_cache.get();
                            fit_data.lr_shield.add_entry(item_key, effect.key, rep_getter);
                            if effect.charge.is_some() {
                                fit_data.lr_shield_limitable.add_entry(item_key, effect.key, rep_getter);
                            }
                        }
                        if let Some(rep_getter) = effect.local_armor_rep_opc_getter {
                            let fit_data = fd_cache.get();
                            fit_data.lr_armor.add_entry(item_key, effect.key, rep_getter);
                            if effect.charge.is_some() {
                                fit_data.lr_armor_limitable.add_entry(item_key, effect.key, rep_getter);
                            }
                        }
                        if let Some(rep_getter) = effect.local_hull_rep_opc_getter {
                            let fit_data = fd_cache.get();
                            fit_data.lr_hull.add_entry(item_key, effect.key, rep_getter);
                        }
                        // Outgoing reps
                        handle_orrs_start(&mut fd_cache, effect, item_key);
                        // Cap
                        if let Some(cap_inject_getter) = effect.cap_inject_getter {
                            let fit_data = fd_cache.get();
                            fit_data.cap_injects.add_entry(item_key, effect.key, cap_inject_getter);
                        }
                        handle_neut_start(&mut fd_cache, effect, item_key);
                        if let Some(use_attr_key) = effect.discharge_attr_key {
                            let fit_data = fd_cache.get();
                            fit_data
                                .cap_consumers_active
                                .add_entry(item_key, effect.key, use_attr_key);
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
                let mut fd_cache = FDCache::new(&mut self.fit_datas, autocharge.get_fit_key());
                for effect in effects {
                    handle_aggr_stop(&mut fd_cache, effect, item_key);
                    if effect.is_active() {
                        handle_dmg_stop(&mut fd_cache, effect, item_key);
                    }
                }
            }
            UItem::Charge(charge) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, charge.get_fit_key());
                for effect in effects {
                    handle_aggr_stop(&mut fd_cache, effect, item_key);
                    if effect.is_active() {
                        handle_dmg_stop(&mut fd_cache, effect, item_key);
                        handle_neut_stop(&mut fd_cache, effect, item_key);
                    }
                }
            }
            UItem::Drone(drone) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, drone.get_fit_key());
                for effect in effects {
                    handle_aggr_stop(&mut fd_cache, effect, item_key);
                    if effect.is_active_with_duration {
                        handle_dmg_stop(&mut fd_cache, effect, item_key);
                        handle_mining_stop(&mut fd_cache, effect, item_key);
                        handle_orrs_stop(&mut fd_cache, effect, item_key);
                        handle_neut_stop(&mut fd_cache, effect, item_key);
                    }
                }
            }
            UItem::Fighter(fighter) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, fighter.get_fit_key());
                for effect in effects {
                    handle_aggr_stop(&mut fd_cache, effect, item_key);
                    if effect.is_active_with_duration {
                        handle_dmg_stop(&mut fd_cache, effect, item_key);
                        handle_orrs_stop(&mut fd_cache, effect, item_key);
                        handle_neut_stop(&mut fd_cache, effect, item_key);
                    }
                    for effect in effects {
                        if effect.banned_in_hisec || effect.banned_in_lowsec {
                            let fit_data = fd_cache.get();
                            fit_data.sec_zone_effect.remove_l2(item_key, &effect.key);
                        }
                    }
                }
            }
            UItem::Module(module) => {
                let mut fd_cache = FDCache::new(&mut self.fit_datas, module.get_fit_key());
                for effect in effects {
                    handle_aggr_stop(&mut fd_cache, effect, item_key);
                    if effect.is_active_with_duration {
                        handle_dmg_stop(&mut fd_cache, effect, item_key);
                        handle_mining_stop(&mut fd_cache, effect, item_key);
                        // Local reps
                        if effect.local_shield_rep_opc_getter.is_some() {
                            let fit_data = fd_cache.get();
                            fit_data.lr_shield.remove_l2(item_key, &effect.key);
                            if effect.charge.is_some() {
                                fit_data.lr_shield_limitable.remove_l2(item_key, &effect.key);
                            }
                        }
                        if effect.local_armor_rep_opc_getter.is_some() {
                            let fit_data = fd_cache.get();
                            fit_data.lr_armor.remove_l2(item_key, &effect.key);
                            if effect.charge.is_some() {
                                fit_data.lr_armor_limitable.remove_l2(item_key, &effect.key);
                            }
                        }
                        if effect.local_hull_rep_opc_getter.is_some() {
                            let fit_data = fd_cache.get();
                            fit_data.lr_hull.remove_l2(item_key, &effect.key);
                        }
                        // Outgoing reps
                        handle_orrs_stop(&mut fd_cache, effect, item_key);
                        // Cap
                        if effect.cap_inject_getter.is_some() {
                            let fit_data = fd_cache.get();
                            fit_data.cap_injects.remove_l2(item_key, &effect.key);
                        }
                        handle_neut_stop(&mut fd_cache, effect, item_key);
                        if effect.discharge_attr_key.is_some() {
                            let fit_data = fd_cache.get();
                            fit_data.cap_consumers_active.remove_l2(item_key, &effect.key);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn handle_aggr_start(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if effect.is_offense {
        let fit_data = fd_cache.get();
        fit_data.aggro_effects.insert(EffectSpec::new(item_key, effect.key));
    }
}
fn handle_aggr_stop(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if effect.is_offense {
        let fit_data = fd_cache.get();
        fit_data.aggro_effects.remove(&EffectSpec::new(item_key, effect.key));
    }
}

fn handle_dmg_start(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if let Some(dmg_getter) = effect.normal_dmg_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.dmg_normal.add_entry(item_key, effect.key, dmg_getter);
    }
    if let Some(dmg_getter) = effect.breacher_dmg_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.dmg_breacher.add_entry(item_key, effect.key, dmg_getter);
    }
}
fn handle_dmg_stop(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if effect.normal_dmg_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.dmg_normal.remove_l2(item_key, &effect.key);
    }
    if effect.breacher_dmg_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.dmg_breacher.remove_l2(item_key, &effect.key);
    }
}

fn handle_mining_start(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if let Some(mining_getter) = effect.mining_ore_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.mining_ore.add_entry(item_key, effect.key, mining_getter);
    }
    if let Some(mining_getter) = effect.mining_ice_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.mining_ice.add_entry(item_key, effect.key, mining_getter);
    }
    if let Some(mining_getter) = effect.mining_gas_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.mining_gas.add_entry(item_key, effect.key, mining_getter);
    }
}
fn handle_mining_stop(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if effect.mining_ore_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.mining_ore.remove_l2(item_key, &effect.key);
    }
    if effect.mining_ice_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.mining_ice.remove_l2(item_key, &effect.key);
    }
    if effect.mining_gas_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.mining_gas.remove_l2(item_key, &effect.key);
    }
}

fn handle_neut_start(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if let Some(neut_getter) = effect.neut_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.out_neuts.add_entry(item_key, effect.key, neut_getter);
    }
}
fn handle_neut_stop(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if effect.neut_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.out_neuts.remove_l2(item_key, &effect.key);
    }
}

fn handle_orrs_start(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if let Some(rep_getter) = effect.outgoing_shield_rep_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.orr_shield.add_entry(item_key, effect.key, rep_getter);
    }
    if let Some(rep_getter) = effect.outgoing_armor_rep_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.orr_armor.add_entry(item_key, effect.key, rep_getter);
    }
    if let Some(rep_getter) = effect.outgoing_hull_rep_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.orr_hull.add_entry(item_key, effect.key, rep_getter);
    }
    if let Some(rep_getter) = effect.outgoing_cap_rep_opc_getter {
        let fit_data = fd_cache.get();
        fit_data.out_cap.add_entry(item_key, effect.key, rep_getter);
    }
}
fn handle_orrs_stop(fd_cache: &mut FDCache, effect: &RcEffect, item_key: UItemKey) {
    if effect.outgoing_shield_rep_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.orr_shield.remove_l2(item_key, &effect.key);
    }
    if effect.outgoing_armor_rep_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.orr_armor.remove_l2(item_key, &effect.key);
    }
    if effect.outgoing_hull_rep_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.orr_hull.remove_l2(item_key, &effect.key);
    }
    if effect.outgoing_cap_rep_opc_getter.is_some() {
        let fit_data = fd_cache.get();
        fit_data.out_cap.remove_l2(item_key, &effect.key);
    }
}
