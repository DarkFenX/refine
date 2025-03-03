use itertools::Itertools;

use crate::{
    adg::{
        GData, GSupport,
        rels::{Fk, KeyDb, KeyPart, Pk},
    },
    ec, ed,
    util::{Named, StMap, StSet},
};

/// Ensure that assumptions reefast makes about the data are true.
pub(in crate::adg) fn validate(g_data: &mut GData, g_supp: &GSupport) {
    fk_check(g_data, g_supp);
    default_effects(g_data);
    known_fighter_abilities(g_data);
    fighter_ability_effect(g_data);
}

/// FK validity. Strictly speaking, not needed for the engine, but reporting data inconsistencies is
/// a good idea, since it can help trace down the case when some adapted type fails to load.
fn fk_check(g_data: &GData, g_supp: &GSupport) {
    let pkdb = KeyDb::new_pkdb(g_data);
    fk_check_referer(&g_data.items, &pkdb, g_supp);
    fk_check_referer(&g_data.groups, &pkdb, g_supp);
    fk_check_referer(&g_data.attrs, &pkdb, g_supp);
    fk_check_referer(&g_data.item_attrs, &pkdb, g_supp);
    fk_check_referer(&g_data.effects, &pkdb, g_supp);
    fk_check_referer(&g_data.item_effects, &pkdb, g_supp);
    fk_check_referer(&g_data.abils, &pkdb, g_supp);
    fk_check_referer(&g_data.item_abils, &pkdb, g_supp);
    fk_check_referer(&g_data.buffs, &pkdb, g_supp);
    fk_check_referer(&g_data.item_srqs, &pkdb, g_supp);
    fk_check_referer(&g_data.muta_items, &pkdb, g_supp);
    fk_check_referer(&g_data.muta_attrs, &pkdb, g_supp);
}
fn fk_check_referer<T: Fk + Named>(rer_vec: &[T], pkdb: &KeyDb, g_supp: &GSupport) {
    fk_check_referee(rer_vec, &pkdb.items, g_supp, T::get_item_fks, ed::EItem::get_name());
    fk_check_referee(
        rer_vec,
        &pkdb.groups,
        g_supp,
        T::get_group_fks,
        ed::EItemGroup::get_name(),
    );
    fk_check_referee(rer_vec, &pkdb.attrs, g_supp, T::get_attr_fks, ed::EAttr::get_name());
    fk_check_referee(
        rer_vec,
        &pkdb.effects,
        g_supp,
        T::get_effect_fks,
        ed::EEffect::get_name(),
    );
    fk_check_referee(
        rer_vec,
        &pkdb.abils,
        g_supp,
        T::get_abil_fks,
        ed::EFighterAbil::get_name(),
    );
    fk_check_referee(rer_vec, &pkdb.buffs, g_supp, T::get_buff_fks, ed::EBuff::get_name());
}
fn fk_check_referee<T, F>(rer_vec: &[T], ree_pks: &StSet<KeyPart>, g_supp: &GSupport, func: F, ree_name: &str)
where
    T: Fk + Named,
    F: Fn(&T, &GSupport) -> Vec<KeyPart>,
{
    let mut fks = StSet::new();
    rer_vec.iter().for_each(|v| fks.extend(func(v, g_supp)));
    let missing = fks.difference(ree_pks).collect_vec();
    if !missing.is_empty() {
        let msg = format!(
            "{} refers to {} missing {}: {}",
            T::get_name(),
            missing.len(),
            ree_name,
            missing.iter().sorted().join(", ")
        );
        tracing::warn!("{msg}");
    }
}

/// One default effect per item max. Needed for adapted item generation.
fn default_effects(g_data: &mut GData) {
    let mut unsets = 0;
    let mut seen_defeffs = StSet::new();
    for e_item_effect in g_data.item_effects.iter_mut() {
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

/// Remove unknown fighter abilities.
fn known_fighter_abilities(g_data: &mut GData) {
    let mut unknown_ids = StSet::new();
    let abils = g_data
        .abils
        .extract_if(.., |v| ec::extras::get_abil_effect(v.id).is_none())
        .update(|v| {
            unknown_ids.insert(v.id);
        })
        .count();
    let item_abils = g_data
        .item_abils
        .extract_if(.., |v| ec::extras::get_abil_effect(v.abil_id).is_none())
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
        tracing::warn!("{msg}");
    }
}

/// Remove item abilities which have no effects to handle them.
fn fighter_ability_effect(g_data: &mut GData) {
    let mut item_eff_map = StMap::new();
    for item_eff in g_data.item_effects.iter() {
        item_eff_map
            .entry(item_eff.item_id)
            .or_insert_with(StSet::new)
            .insert(item_eff.effect_id);
    }
    let mut invalids = StSet::new();
    g_data
        .item_abils
        .extract_if(.., |v| match ec::extras::get_abil_effect(v.abil_id) {
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
        tracing::warn!("{msg}");
    }
}
