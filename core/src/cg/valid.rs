use std::collections::{HashMap, HashSet};

use log;

use crate::{
    cg::data::{Fk, KeyDb, Pk, Support},
    defines::ReeInt,
    dh,
    util::Named,
};

use super::Data;

/// Ensure that assumptions the crate makes about the data are true.
///
/// Cachable type generation and the crate operation relies on several assumptions, which are
/// possible to break with the data handling format the crate exposes.
pub(super) fn validate(data: &mut Data, supp: &Support, errs: &mut Vec<String>) {
    fk_check(data, errs, supp);
    default_effects(data, errs);
    item_dynamics(data, errs);
}

/// FK validity. Strictly speaking, not needed for the engine, but reporting data consistency errors
/// is a good idea since it can help trace down the case when something fails to load from cache.
fn fk_check(data: &Data, errs: &mut Vec<String>, supp: &Support) {
    let pkdb = KeyDb::new_pkdb(data);
    fk_check_referer(&data.items, &pkdb, supp, errs);
    fk_check_referer(&data.groups, &pkdb, supp, errs);
    fk_check_referer(&data.attrs, &pkdb, supp, errs);
    fk_check_referer(&data.item_attrs, &pkdb, supp, errs);
    fk_check_referer(&data.effects, &pkdb, supp, errs);
    fk_check_referer(&data.item_effects, &pkdb, supp, errs);
    fk_check_referer(&data.abils, &pkdb, supp, errs);
    fk_check_referer(&data.item_abils, &pkdb, supp, errs);
    fk_check_referer(&data.buffs, &pkdb, supp, errs);
    fk_check_referer(&data.item_srqs, &pkdb, supp, errs);
    fk_check_referer(&data.muta_items, &pkdb, supp, errs);
    fk_check_referer(&data.muta_attrs, &pkdb, supp, errs);
}
fn fk_check_referer<T>(rer_vec: &Vec<T>, pkdb: &KeyDb, supp: &Support, errs: &mut Vec<String>)
where
    T: Fk + Named,
{
    fk_check_referee(rer_vec, &pkdb.items, supp, T::get_item_fks, dh::Item::get_name(), errs);
    fk_check_referee(
        rer_vec,
        &pkdb.groups,
        supp,
        T::get_group_fks,
        dh::ItemGroup::get_name(),
        errs,
    );
    fk_check_referee(rer_vec, &pkdb.attrs, supp, T::get_attr_fks, dh::Attr::get_name(), errs);
    fk_check_referee(
        rer_vec,
        &pkdb.effects,
        supp,
        T::get_effect_fks,
        dh::Effect::get_name(),
        errs,
    );
    fk_check_referee(
        rer_vec,
        &pkdb.abils,
        supp,
        T::get_abil_fks,
        dh::FighterAbil::get_name(),
        errs,
    );
    fk_check_referee(rer_vec, &pkdb.buffs, supp, T::get_buff_fks, dh::Buff::get_name(), errs);
}
fn fk_check_referee<T, F>(
    rer_vec: &Vec<T>,
    ree_pks: &HashSet<ReeInt>,
    supp: &Support,
    func: F,
    ree_name: &str,
    errs: &mut Vec<String>,
) where
    T: Fk + Named,
    F: Fn(&T, &Support) -> Vec<ReeInt>,
{
    let mut fks = HashSet::new();
    rer_vec.iter().map(|v| fks.extend(func(v, supp))).for_each(drop);
    let mut missing: Vec<_> = fks.difference(ree_pks).collect();
    if missing.len() > 0 {
        missing.sort_unstable();
        let msg = format!(
            "{} refers to {} missing {}: {}",
            T::get_name(),
            missing.len(),
            ree_name,
            missing
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        log::warn!("{}", &msg);
        errs.push(msg);
    }
}

/// One default effect per item max. Needed for Item generation.
fn default_effects(data: &mut Data, errs: &mut Vec<String>) {
    let mut unsets = 0;
    let mut seen_des = HashSet::new();
    for item_effect in data.item_effects.iter_mut() {
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
        errs.push(msg);
    }
}

/// All mutaplasmids are non-dynamic, take non-dynamic items and produce dynamic items.
fn item_dynamics(data: &mut Data, errs: &mut Vec<String>) {
    let mut dynmap = HashMap::new();
    data.items
        .iter()
        .map(|v| dynmap.insert(v.id, v.is_dynamic))
        .for_each(drop);
    let getdyn = |id| dynmap.get(&id).unwrap_or(&false).to_owned();
    let removed = data
        .muta_items
        .drain_filter(|v| getdyn(v.muta_id) || getdyn(v.in_item_id) || !getdyn(v.out_item_id))
        .count();
    if removed > 0 {
        let msg = format!(
            "removed {} mutaplasmid item conversions due to wrong \"dynamic\" flag on referenced item",
            removed
        );
        log::warn!("{}", &msg);
        errs.push(msg);
    }
}
