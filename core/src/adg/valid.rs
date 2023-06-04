use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use log;

use crate::{
    adg::{
        rels::{Fk, KeyDb, Pk},
        GData, GSupport,
    },
    consts,
    defs::ReeInt,
    ed,
    util::Named,
};

/// Ensure that assumptions reefast makes about the data are true.
pub(in crate::adg) fn validate(gdata: &mut GData, gsupp: &GSupport) {
    fk_check(gdata, gsupp);
    default_effects(gdata);
    known_fighter_abilities(gdata);
    fighter_ability_effect(gdata);
}

/// FK validity. Strictly speaking, not needed for the engine, but reporting data inconsistencies is
/// a good idea, since it can help trace down the case when some adapted type fails to load.
fn fk_check(gdata: &GData, gsupp: &GSupport) {
    let pkdb = KeyDb::new_pkdb(gdata);
    fk_check_referer(&gdata.items, &pkdb, gsupp);
    fk_check_referer(&gdata.groups, &pkdb, gsupp);
    fk_check_referer(&gdata.attrs, &pkdb, gsupp);
    fk_check_referer(&gdata.item_attrs, &pkdb, gsupp);
    fk_check_referer(&gdata.effects, &pkdb, gsupp);
    fk_check_referer(&gdata.item_effects, &pkdb, gsupp);
    fk_check_referer(&gdata.abils, &pkdb, gsupp);
    fk_check_referer(&gdata.item_abils, &pkdb, gsupp);
    fk_check_referer(&gdata.buffs, &pkdb, gsupp);
    fk_check_referer(&gdata.item_srqs, &pkdb, gsupp);
    fk_check_referer(&gdata.muta_items, &pkdb, gsupp);
    fk_check_referer(&gdata.muta_attrs, &pkdb, gsupp);
}
fn fk_check_referer<T: Fk + Named>(rer_vec: &Vec<T>, pkdb: &KeyDb, gsupp: &GSupport) {
    fk_check_referee(rer_vec, &pkdb.items, gsupp, T::get_item_fks, ed::EItem::get_name());
    fk_check_referee(
        rer_vec,
        &pkdb.groups,
        gsupp,
        T::get_group_fks,
        ed::EItemGroup::get_name(),
    );
    fk_check_referee(rer_vec, &pkdb.attrs, gsupp, T::get_attr_fks, ed::EAttr::get_name());
    fk_check_referee(
        rer_vec,
        &pkdb.effects,
        gsupp,
        T::get_effect_fks,
        ed::EEffect::get_name(),
    );
    fk_check_referee(
        rer_vec,
        &pkdb.abils,
        gsupp,
        T::get_abil_fks,
        ed::EFighterAbil::get_name(),
    );
    fk_check_referee(rer_vec, &pkdb.buffs, gsupp, T::get_buff_fks, ed::EBuff::get_name());
}
fn fk_check_referee<T, F>(rer_vec: &Vec<T>, ree_pks: &HashSet<ReeInt>, gsupp: &GSupport, func: F, ree_name: &str)
where
    T: Fk + Named,
    F: Fn(&T, &GSupport) -> Vec<ReeInt>,
{
    let mut fks = HashSet::new();
    rer_vec.iter().for_each(|v| fks.extend(func(v, gsupp)));
    let missing = fks.difference(ree_pks).collect_vec();
    if missing.len() > 0 {
        let msg = format!(
            "{} refers to {} missing {}: {}",
            T::get_name(),
            missing.len(),
            ree_name,
            missing.iter().sorted().join(", ")
        );
        log::warn!("{}", msg);
    }
}

/// One default effect per item max. Needed for adapted item generation.
fn default_effects(gdata: &mut GData) {
    let mut unsets = 0;
    let mut seen_des = HashSet::new();
    for item_effect in gdata.item_effects.iter_mut() {
        if item_effect.is_default {
            if !seen_des.insert(item_effect.get_pk()) {
                unsets += 1;
                item_effect.is_default = false
            }
        }
    }
    if unsets > 0 {
        let msg = format!("set {} excessive default effects as non-default", unsets);
        log::warn!("{}", msg);
    }
}

/// Remove unknown fighter abilities.
fn known_fighter_abilities(gdata: &mut GData) {
    let mut unknown_ids = HashSet::new();
    let abils = gdata
        .abils
        .drain_filter(|v| consts::get_abil_effect(v.id).is_none())
        .update(|v| {
            unknown_ids.insert(v.id);
        })
        .count();
    let item_abils = gdata
        .item_abils
        .drain_filter(|v| consts::get_abil_effect(v.abil_id).is_none())
        .update(|v| {
            unknown_ids.insert(v.abil_id);
        })
        .count();
    if abils > 0 || item_abils > 0 {
        let msg = format!(
            "removed {} {} and {} {} with unknown fighter ability IDs: {}",
            abils,
            ed::EFighterAbil::get_name(),
            item_abils,
            ed::EItemFighterAbil::get_name(),
            unknown_ids.iter().sorted().join(", ")
        );
        log::warn!("{}", msg);
    }
}

/// Remove item abilities which have no effects to handle them.
fn fighter_ability_effect(gdata: &mut GData) {
    let mut item_eff_map = HashMap::new();
    for item_eff in gdata.item_effects.iter() {
        item_eff_map
            .entry(item_eff.item_id)
            .or_insert_with(|| HashSet::new())
            .insert(item_eff.effect_id);
    }
    let mut invalids = HashSet::new();
    gdata
        .item_abils
        .drain_filter(|v| match consts::get_abil_effect(v.abil_id) {
            Some(eid) => match item_eff_map.get(&v.item_id) {
                Some(eids) => !eids.contains(&eid),
                None => true,
            },
            None => true,
        })
        .for_each(|v| {
            invalids.insert((v.item_id, v.abil_id));
        });
    if invalids.len() > 0 {
        let max_logged = 5;
        let msg = format!(
            "removed {} {} with references to missing effects, showing up to {}: {}",
            invalids.len(),
            ed::EItemFighterAbil::get_name(),
            max_logged,
            invalids
                .iter()
                .sorted()
                .take(max_logged)
                .format_with(", ", |v, f| f(&format_args!("[{}, {}]", v.0, v.1)))
        );
        log::warn!("{}", msg);
    }
}
