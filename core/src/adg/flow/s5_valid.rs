use itertools::Itertools;

use crate::{
    adg::{
        GSupport, get_abil_effect,
        rels::{Fk, KeyDb, KeyPart, Pk},
    },
    ed::{EAttr, EBuff, EData, EDataCont, EEffect, EFighterAbil, EItem, EItemFighterAbil, EItemGroup, EItemList},
    util::{Named, RMap, RSet},
};

/// Ensure that assumptions reefast makes about the data are true.
pub(in crate::adg) fn validate(e_data: &mut EData, g_supp: &GSupport) {
    fk_check(e_data, g_supp);
    default_effects(e_data);
    unmapped_abilities(e_data);
    broken_ability_links(e_data);
    item_ability_handler_effect(e_data);
}

/// FK validity. Strictly speaking, not needed for the engine, but reporting data inconsistencies is
/// a good idea, since it can help trace down the case when some adapted type fails to load.
fn fk_check(e_data: &EData, g_supp: &GSupport) {
    let pkdb = KeyDb::new_pkdb(e_data);
    fk_check_referer(&e_data.items, &pkdb, g_supp);
    fk_check_referer(&e_data.groups, &pkdb, g_supp);
    fk_check_referer(&e_data.item_lists, &pkdb, g_supp);
    fk_check_referer(&e_data.attrs, &pkdb, g_supp);
    fk_check_referer(&e_data.item_attrs, &pkdb, g_supp);
    fk_check_referer(&e_data.effects, &pkdb, g_supp);
    fk_check_referer(&e_data.item_effects, &pkdb, g_supp);
    fk_check_referer(&e_data.abils, &pkdb, g_supp);
    fk_check_referer(&e_data.item_abils, &pkdb, g_supp);
    fk_check_referer(&e_data.buffs, &pkdb, g_supp);
    fk_check_referer(&e_data.space_comps, &pkdb, g_supp);
    fk_check_referer(&e_data.item_srqs, &pkdb, g_supp);
    fk_check_referer(&e_data.muta_items, &pkdb, g_supp);
    fk_check_referer(&e_data.muta_attrs, &pkdb, g_supp);
}
fn fk_check_referer<T: Fk + Named>(rer_cont: &EDataCont<T>, pkdb: &KeyDb, g_supp: &GSupport) {
    fk_check_referee(rer_cont, &pkdb.items, g_supp, T::get_item_fks, EItem::get_name());
    fk_check_referee(rer_cont, &pkdb.groups, g_supp, T::get_group_fks, EItemGroup::get_name());
    fk_check_referee(
        rer_cont,
        &pkdb.item_lists,
        g_supp,
        T::get_item_list_fks,
        EItemList::get_name(),
    );
    fk_check_referee(rer_cont, &pkdb.attrs, g_supp, T::get_attr_fks, EAttr::get_name());
    fk_check_referee(rer_cont, &pkdb.effects, g_supp, T::get_effect_fks, EEffect::get_name());
    fk_check_referee(rer_cont, &pkdb.abils, g_supp, T::get_abil_fks, EFighterAbil::get_name());
    fk_check_referee(rer_cont, &pkdb.buffs, g_supp, T::get_buff_fks, EBuff::get_name());
}
fn fk_check_referee<T, F>(rer_cont: &EDataCont<T>, ree_pks: &RSet<KeyPart>, g_supp: &GSupport, func: F, ree_name: &str)
where
    T: Fk + Named,
    F: Fn(&T, &GSupport) -> Vec<KeyPart>,
{
    let mut fks = RSet::new();
    rer_cont.data.iter().for_each(|v| fks.extend(func(v, g_supp)));
    let missing = fks.difference(ree_pks).collect_vec();
    if !missing.is_empty() {
        let msg = format!(
            "{} refers to {} missing {}: {}",
            T::get_name(),
            missing.len(),
            ree_name,
            missing.iter().sorted_unstable().join(", ")
        );
        tracing::warn!("{msg}");
    }
}

/// One default effect per item max. Needed for adapted item generation.
fn default_effects(e_data: &mut EData) {
    let mut unsets = 0;
    let mut seen_defeffs = RSet::new();
    for e_item_effect in e_data.item_effects.data.iter_mut() {
        if e_item_effect.is_default && !seen_defeffs.insert(e_item_effect.get_pk()) {
            unsets += 1;
            e_item_effect.is_default = false
        }
    }
    if unsets > 0 {
        let msg = format!("set {unsets} excessive default effects as non-default");
        tracing::warn!("{msg}");
    }
}

/// Remove fighter abilities which cannot be mapped to existing effect.
fn unmapped_abilities(e_data: &mut EData) {
    let effect_ids: RSet<_> = e_data.effects.data.iter().map(|v| v.id).collect();
    let mut unknown_ids = RSet::new();
    let abils = e_data
        .abils
        .data
        .extract_if(.., |v| match get_abil_effect(v.id) {
            Some(effect_id) => !effect_ids.contains(&effect_id),
            None => true,
        })
        .inspect(|v| {
            unknown_ids.insert(v.id);
        })
        .count();
    let item_abils = e_data
        .item_abils
        .data
        .extract_if(.., |v| match get_abil_effect(v.abil_id) {
            Some(effect_id) => !effect_ids.contains(&effect_id),
            None => true,
        })
        .inspect(|v| {
            unknown_ids.insert(v.abil_id);
        })
        .count();
    if abils > 0 || item_abils > 0 {
        let max_logged = 5;
        let msg = format!(
            "removed {} {} and {} {} with unmappable fighter ability IDs, showing up to {}: {}",
            abils,
            EFighterAbil::get_name(),
            item_abils,
            EItemFighterAbil::get_name(),
            max_logged,
            unknown_ids.iter().sorted_unstable().take(max_logged).join(", ")
        );
        tracing::warn!("{msg}");
    }
}

/// Remove fighter abilities which do not have corresponding ability entry.
fn broken_ability_links(e_data: &mut EData) {
    let abil_ids: RSet<_> = e_data.abils.data.iter().map(|v| v.id).collect();
    let mut broken_ids = RSet::new();
    let item_abils = e_data
        .item_abils
        .data
        .extract_if(.., |v| !abil_ids.contains(&v.abil_id))
        .inspect(|v| {
            broken_ids.insert(v.abil_id);
        })
        .count();
    if !broken_ids.is_empty() {
        let max_logged = 5;
        let msg = format!(
            "removed {} {} with invalid target {}, showing up to {}: {}",
            item_abils,
            EItemFighterAbil::get_name(),
            EFighterAbil::get_name(),
            max_logged,
            broken_ids.iter().sorted_unstable().take(max_logged).join(", ")
        );
        tracing::warn!("{msg}");
    }
}

/// Remove item abilities which have no effect on item to handle them.
fn item_ability_handler_effect(e_data: &mut EData) {
    let mut item_eff_map = RMap::new();
    for item_eff in e_data.item_effects.data.iter() {
        item_eff_map
            .entry(item_eff.item_id)
            .or_insert_with(RSet::new)
            .insert(item_eff.effect_id);
    }
    let mut invalids = RSet::new();
    e_data
        .item_abils
        .data
        .extract_if(.., |v| match get_abil_effect(v.abil_id) {
            Some(eid) => match item_eff_map.get(&v.item_id) {
                Some(eids) => !eids.contains(&eid),
                None => true,
            },
            None => true,
        })
        .for_each(|v| {
            invalids.insert((v.item_id, v.abil_id));
        });
    if !invalids.is_empty() {
        let max_logged = 5;
        let msg = format!(
            "removed {} {} with references to missing on-item effects, showing up to {}: {}",
            invalids.len(),
            EItemFighterAbil::get_name(),
            max_logged,
            invalids
                .iter()
                .sorted_unstable()
                .take(max_logged)
                .format_with(", ", |v, f| f(&format_args!("[{}, {}]", v.0, v.1)))
        );
        tracing::warn!("{msg}");
    }
}
