use std::collections::{HashMap, HashSet};

use itertools::Itertools;
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
pub(super) fn validate(data: &mut Data, supp: &Support, warns: &mut Vec<String>) {
    fk_check(data, warns, supp);
    default_effects(data, warns);
}

/// FK validity. Strictly speaking, not needed for the engine, but reporting data inconsistencies is
/// a good idea, since it can help trace down the case when something fails to load from cache
/// later.
fn fk_check(data: &Data, warns: &mut Vec<String>, supp: &Support) {
    let pkdb = KeyDb::new_pkdb(data);
    fk_check_referer(&data.items, &pkdb, supp, warns);
    fk_check_referer(&data.groups, &pkdb, supp, warns);
    fk_check_referer(&data.attrs, &pkdb, supp, warns);
    fk_check_referer(&data.item_attrs, &pkdb, supp, warns);
    fk_check_referer(&data.effects, &pkdb, supp, warns);
    fk_check_referer(&data.item_effects, &pkdb, supp, warns);
    fk_check_referer(&data.abils, &pkdb, supp, warns);
    fk_check_referer(&data.item_abils, &pkdb, supp, warns);
    fk_check_referer(&data.buffs, &pkdb, supp, warns);
    fk_check_referer(&data.item_srqs, &pkdb, supp, warns);
    fk_check_referer(&data.muta_items, &pkdb, supp, warns);
    fk_check_referer(&data.muta_attrs, &pkdb, supp, warns);
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
fn default_effects(data: &mut Data, warns: &mut Vec<String>) {
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
        warns.push(msg);
    }
}
