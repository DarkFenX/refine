use itertools::Itertools;

use crate::{
    ad::{
        ADataGenerator, AdgWarnings,
        generator::{
            AdgSupport, get_abil_effect,
            rels::{Fk, KeyDb, KeyPart, Pk},
        },
    },
    ed::{EAbil, EAttr, EBuff, EDataCont, EEffect, EItem, EItemAbil, EItemGroup, EItemList},
    util::{LibNamed, RMap, RSet},
};

impl ADataGenerator {
    /// Ensure that assumptions refine makes about the data are true.
    pub(in crate::ad::generator) fn validate(&mut self) {
        self.fk_check();
        self.default_effects();
        self.unmapped_abilities();
        self.broken_ability_links();
        self.item_ability_handler_effect();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// FK validity - strictly speaking, not needed for the engine, but reporting data inconsistencies is
// a good idea, since it can help trace down the case when some adapted type fails to load.
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn fk_check(&mut self) {
        let pkdb = KeyDb::new_pkdb(&self.e_data);
        fk_check_referer(&self.e_data.items, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.groups, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.item_lists, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.attrs, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.item_attrs, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.effects, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(
            &self.e_data.item_effects,
            &pkdb,
            &self.support,
            &mut self.a_data.warnings,
        );
        fk_check_referer(&self.e_data.abils, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.item_abils, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.buffs, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(
            &self.e_data.space_comps,
            &pkdb,
            &self.support,
            &mut self.a_data.warnings,
        );
        fk_check_referer(&self.e_data.item_srqs, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.muta_items, &pkdb, &self.support, &mut self.a_data.warnings);
        fk_check_referer(&self.e_data.muta_attrs, &pkdb, &self.support, &mut self.a_data.warnings);
    }
}
fn fk_check_referer<T>(rer_cont: &EDataCont<T>, pkdb: &KeyDb, adg_supp: &AdgSupport, a_warnings: &mut AdgWarnings)
where
    T: Fk + LibNamed,
{
    fk_check_referee(
        rer_cont,
        &pkdb.items,
        adg_supp,
        a_warnings,
        T::get_item_fks,
        EItem::lib_get_name(),
    );
    fk_check_referee(
        rer_cont,
        &pkdb.groups,
        adg_supp,
        a_warnings,
        T::get_group_fks,
        EItemGroup::lib_get_name(),
    );
    fk_check_referee(
        rer_cont,
        &pkdb.item_lists,
        adg_supp,
        a_warnings,
        T::get_item_list_fks,
        EItemList::lib_get_name(),
    );
    fk_check_referee(
        rer_cont,
        &pkdb.attrs,
        adg_supp,
        a_warnings,
        T::get_attr_fks,
        EAttr::lib_get_name(),
    );
    fk_check_referee(
        rer_cont,
        &pkdb.effects,
        adg_supp,
        a_warnings,
        T::get_effect_fks,
        EEffect::lib_get_name(),
    );
    fk_check_referee(
        rer_cont,
        &pkdb.abils,
        adg_supp,
        a_warnings,
        T::get_abil_fks,
        EAbil::lib_get_name(),
    );
    fk_check_referee(
        rer_cont,
        &pkdb.buffs,
        adg_supp,
        a_warnings,
        T::get_buff_fks,
        EBuff::lib_get_name(),
    );
}
fn fk_check_referee<T, F>(
    referer_cont: &EDataCont<T>,
    referee_pks: &RSet<KeyPart>,
    adg_supp: &AdgSupport,
    a_warnings: &mut AdgWarnings,
    func: F,
    referee_name: &str,
) where
    T: Fk + LibNamed,
    F: Fn(&T, &AdgSupport) -> Vec<KeyPart>,
{
    let mut fks = RSet::new();
    referer_cont.data.iter().for_each(|v| fks.extend(func(v, adg_supp)));
    let missing = fks.difference(referee_pks).copied().collect_vec();
    if !missing.is_empty() {
        let warning = format!(
            "{} refers to {} missing {}: {}",
            T::lib_get_name(),
            missing.len(),
            referee_name,
            missing.into_iter().map(|v| v.into_i32()).sorted_unstable().join(", ")
        );
        a_warnings.validation.push(warning);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// One default effect per item max - needed for adapted item generation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn default_effects(&mut self) {
        let mut unsets = 0;
        let mut seen_defeffs = RSet::new();
        for e_item_effect in self.e_data.item_effects.data.iter_mut() {
            if e_item_effect.is_default && !seen_defeffs.insert(e_item_effect.get_pk()) {
                unsets += 1;
                e_item_effect.is_default = false
            }
        }
        if unsets > 0 {
            let warning = format!("set {unsets} excessive default effects as non-default");
            self.a_data.warnings.validation.push(warning);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Remove fighter abilities which cannot be mapped to existing effect
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn unmapped_abilities(&mut self) {
        let effect_ids: RSet<_> = self.e_data.effects.data.iter().map(|v| v.id).collect();
        let mut unknown_ids = RSet::new();
        let abils = self
            .e_data
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
        let item_abils = self
            .e_data
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
            let entry_limit = 5;
            let entry_count = unknown_ids.len();
            let mut warning = format!(
                "removed {} {} and {} {} with unmappable fighter ability IDs: {}",
                abils,
                EAbil::lib_get_name(),
                item_abils,
                EItemAbil::lib_get_name(),
                unknown_ids
                    .into_iter()
                    .map(|v| v.into_i32())
                    .sorted_unstable()
                    .take(entry_limit)
                    .join(", ")
            );
            if entry_count > entry_limit {
                warning = format!("{}, <{} more entries hidden>", warning, entry_count - entry_limit);
            }
            self.a_data.warnings.validation.push(warning);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Remove fighter abilities which do not have corresponding ability entry
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn broken_ability_links(&mut self) {
        let abil_ids: RSet<_> = self.e_data.abils.data.iter().map(|v| v.id).collect();
        let mut broken_ids = RSet::new();
        let item_abils = self
            .e_data
            .item_abils
            .data
            .extract_if(.., |v| !abil_ids.contains(&v.abil_id))
            .inspect(|v| {
                broken_ids.insert(v.abil_id);
            })
            .count();
        if !broken_ids.is_empty() {
            let entry_limit = 5;
            let entry_count = broken_ids.len();
            let mut warning = format!(
                "removed {} {} with invalid target {}: {}",
                item_abils,
                EItemAbil::lib_get_name(),
                EAbil::lib_get_name(),
                broken_ids
                    .into_iter()
                    .map(|v| v.into_i32())
                    .sorted_unstable()
                    .take(entry_limit)
                    .join(", ")
            );
            if entry_count > entry_limit {
                warning = format!("{}, <{} more entries hidden>", warning, entry_count - entry_limit);
            }
            self.a_data.warnings.validation.push(warning);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Remove item abilities which have no effect on item to handle them
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn item_ability_handler_effect(&mut self) {
        let mut item_eff_map = RMap::new();
        for item_eff in self.e_data.item_effects.data.iter() {
            item_eff_map
                .entry(item_eff.item_id)
                .or_insert_with(RSet::new)
                .insert(item_eff.effect_id);
        }
        let mut invalids = RSet::new();
        self.e_data
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
            let entry_limit = 5;
            let entry_count = invalids.len();
            let mut warning = format!(
                "removed {} {} with references to missing on-item effects: {}",
                invalids.len(),
                EItemAbil::lib_get_name(),
                invalids
                    .into_iter()
                    .map(|(v1, v2)| (v1.into_i32(), v2.into_i32()))
                    .sorted_unstable()
                    .take(entry_limit)
                    .format_with(", ", |v, f| f(&format_args!("[{}, {}]", v.0, v.1)))
            );
            if entry_count > entry_limit {
                warning = format!("{}, <{} more entries hidden>", warning, entry_count - entry_limit);
            }
            self.a_data.warnings.validation.push(warning);
        }
    }
}
