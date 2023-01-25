use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use log;

use crate::{consts, defines::ReeInt, dh, util::Named};

use super::{
    data::{Fk, KeyDb, Pk, Support},
    CGData,
};

/// Ensure that assumptions REEFAST makes about the data are true.
///
/// See documentation for [`dh`](crate::dh) module about assumptions.
pub(super) fn validate(cg_data: &mut CGData, supp: &Support, warns: &mut Vec<String>) {
    fk_check(cg_data, warns, supp);
    default_effects(cg_data, warns);
    known_fighter_abilities(cg_data, warns);
    fighter_ability_effect(cg_data, warns);
}

/// FK validity. Strictly speaking, not needed for the engine, but reporting data inconsistencies is
/// a good idea, since it can help trace down the case when something fails to load from cache
/// later.
fn fk_check(cg_data: &CGData, warns: &mut Vec<String>, supp: &Support) {
    let pkdb = KeyDb::new_pkdb(cg_data);
    fk_check_referer(&cg_data.items, &pkdb, supp, warns);
    fk_check_referer(&cg_data.groups, &pkdb, supp, warns);
    fk_check_referer(&cg_data.attrs, &pkdb, supp, warns);
    fk_check_referer(&cg_data.item_attrs, &pkdb, supp, warns);
    fk_check_referer(&cg_data.effects, &pkdb, supp, warns);
    fk_check_referer(&cg_data.item_effects, &pkdb, supp, warns);
    fk_check_referer(&cg_data.abils, &pkdb, supp, warns);
    fk_check_referer(&cg_data.item_abils, &pkdb, supp, warns);
    fk_check_referer(&cg_data.buffs, &pkdb, supp, warns);
    fk_check_referer(&cg_data.item_srqs, &pkdb, supp, warns);
    fk_check_referer(&cg_data.muta_items, &pkdb, supp, warns);
    fk_check_referer(&cg_data.muta_attrs, &pkdb, supp, warns);
}
fn fk_check_referer<T>(rer_vec: &Vec<T>, pkdb: &KeyDb, supp: &Support, warns: &mut Vec<String>)
where
    T: Fk + Named,
{
    fk_check_referee(rer_vec, &pkdb.items, supp, T::get_item_fks, dh::Item::get_name(), warns);
    fk_check_referee(
        rer_vec,
        &pkdb.groups,
        supp,
        T::get_group_fks,
        dh::ItemGroup::get_name(),
        warns,
    );
    fk_check_referee(rer_vec, &pkdb.attrs, supp, T::get_attr_fks, dh::Attr::get_name(), warns);
    fk_check_referee(
        rer_vec,
        &pkdb.effects,
        supp,
        T::get_effect_fks,
        dh::Effect::get_name(),
        warns,
    );
    fk_check_referee(
        rer_vec,
        &pkdb.abils,
        supp,
        T::get_abil_fks,
        dh::FighterAbil::get_name(),
        warns,
    );
    fk_check_referee(rer_vec, &pkdb.buffs, supp, T::get_buff_fks, dh::Buff::get_name(), warns);
}
fn fk_check_referee<T, F>(
    rer_vec: &Vec<T>,
    ree_pks: &HashSet<ReeInt>,
    supp: &Support,
    func: F,
    ree_name: &str,
    warns: &mut Vec<String>,
) where
    T: Fk + Named,
    F: Fn(&T, &Support) -> Vec<ReeInt>,
{
    let mut fks = HashSet::new();
    rer_vec.iter().for_each(|v| fks.extend(func(v, supp)));
    let missing = fks.difference(ree_pks).collect_vec();
    if missing.len() > 0 {
        let msg = format!(
            "{} refers to {} missing {}: {}",
            T::get_name(),
            missing.len(),
            ree_name,
            missing.iter().sorted().join(", ")
        );
        log::warn!("{}", &msg);
        warns.push(msg);
    }
}

/// One default effect per item max. Needed for Item generation.
fn default_effects(cg_data: &mut CGData, warns: &mut Vec<String>) {
    let mut unsets = 0;
    let mut seen_des = HashSet::new();
    for item_effect in cg_data.item_effects.iter_mut() {
        if item_effect.is_default {
            if !seen_des.insert(item_effect.get_pk()) {
                unsets += 1;
                item_effect.is_default = false
            }
        }
    }
    if unsets > 0 {
        let msg = format!("set {} excessive default effects as non-default", unsets);
        log::warn!("{}", &msg);
        warns.push(msg);
    }
}

/// Remove unknown fighter abilities.
fn known_fighter_abilities(cg_data: &mut CGData, warns: &mut Vec<String>) {
    let mut unknown_ids = HashSet::new();
    let abils = cg_data
        .abils
        .drain_filter(|v| consts::get_abil_effect(v.id).is_none())
        .update(|v| {
            unknown_ids.insert(v.id);
        })
        .count();
    let item_abils = cg_data
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
            dh::FighterAbil::get_name(),
            item_abils,
            dh::ItemFighterAbil::get_name(),
            unknown_ids.iter().sorted().join(", ")
        );
        log::warn!("{}", &msg);
        warns.push(msg);
    }
}

/// Remove item abilities which have no effects to handle them.
fn fighter_ability_effect(cg_data: &mut CGData, warns: &mut Vec<String>) {
    let mut item_eff_map = HashMap::new();
    for item_eff in cg_data.item_effects.iter() {
        item_eff_map
            .entry(item_eff.item_id)
            .or_insert_with(|| HashSet::new())
            .insert(item_eff.effect_id);
    }
    let mut invalids = HashSet::new();
    cg_data
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
            dh::ItemFighterAbil::get_name(),
            max_logged,
            invalids
                .iter()
                .sorted()
                .take(max_logged)
                .format_with(", ", |v, f| f(&format_args!("[{}, {}]", v.0, v.1)))
        );
        log::warn!("{}", &msg);
        warns.push(msg);
    }
}
