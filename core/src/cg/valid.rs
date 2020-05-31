use std::{collections::HashSet, iter::FromIterator};

use log;

use crate::{
    cg::data::{Fk, KeyDb, Pk, Support},
    consts,
    defines::ReeInt,
    dh,
    util::Named,
};

use super::Data;

/// Ensure that assumptions REEFAST makes about the data are true.
///
/// See documentation for [`dh`](crate::dh) module about assumptions.
pub(super) fn validate(data: &mut Data, supp: &Support, errs: &mut Vec<String>) {
    fk_check(data, errs, supp);
    default_effects(data, errs);
    known_fighter_abilities(data, errs);
}

/// FK validity. Strictly speaking, not needed for the engine, but reporting data consistency errors
/// is a good idea, since it can help trace down the case when something fails to load from cache
/// later.
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
    rer_vec.iter().for_each(|v| fks.extend(func(v, supp)));
    let mut missing: Vec<_> = fks.difference(ree_pks).collect();
    if missing.len() > 0 {
        missing.sort_unstable();
        let msg = format!(
            "{} refers to {} missing {}: {}",
            T::get_name(),
            missing.len(),
            ree_name,
            itertools::join(missing, ", ")
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

/// Remove unknown fighter abilities.
fn known_fighter_abilities(data: &mut Data, errs: &mut Vec<String>) {
    let mut unknown_ids = HashSet::new();
    let abils = data
        .abils
        .drain_filter(|v| consts::get_abil_effect(v.id).is_none())
        .map(|v| {
            unknown_ids.insert(v.id);
            v
        })
        .count();
    let item_abils = data
        .item_abils
        .drain_filter(|v| consts::get_abil_effect(v.abil_id).is_none())
        .map(|v| {
            unknown_ids.insert(v.abil_id);
            v
        })
        .count();
    if abils > 0 || item_abils > 0 {
        let mut unknown_ids = Vec::from_iter(unknown_ids.into_iter());
        unknown_ids.sort_unstable();
        let msg = format!(
            "removed {} {} and {} {} with unknown fighter ability IDs: {}",
            abils,
            dh::FighterAbil::get_name(),
            item_abils,
            dh::ItemFighterAbil::get_name(),
            itertools::join(unknown_ids, ", ")
        );
        log::warn!("{}", &msg);
        errs.push(msg);
    }
}
