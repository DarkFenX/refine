//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use crate::{
    ad::{ADataGenerator, ADataWarnings, generator::rels::Pk},
    ed::{EData, EDataCont},
    util::{LibNamed, RSet},
};

impl ADataGenerator {
    pub(in crate::ad::generator) fn dedup_pks(&mut self) {
        let EData {
            items,
            groups,
            item_lists,
            attrs,
            item_attrs,
            effects,
            item_effects,
            abils,
            item_abils,
            buffs,
            item_buffs,
            muta_items,
            muta_attrs,
        } = &mut self.e_data;
        dedup_pks_vec(items, &mut self.a_data.warnings);
        dedup_pks_vec(groups, &mut self.a_data.warnings);
        dedup_pks_vec(item_lists, &mut self.a_data.warnings);
        dedup_pks_vec(attrs, &mut self.a_data.warnings);
        dedup_pks_vec(item_attrs, &mut self.a_data.warnings);
        dedup_pks_vec(effects, &mut self.a_data.warnings);
        dedup_pks_vec(item_effects, &mut self.a_data.warnings);
        dedup_pks_vec(abils, &mut self.a_data.warnings);
        dedup_pks_vec(item_abils, &mut self.a_data.warnings);
        dedup_pks_vec(buffs, &mut self.a_data.warnings);
        dedup_pks_vec(item_buffs, &mut self.a_data.warnings);
        dedup_pks_vec(muta_items, &mut self.a_data.warnings);
        dedup_pks_vec(muta_attrs, &mut self.a_data.warnings);
    }
}

fn dedup_pks_vec<T: Pk + LibNamed>(e_cont: &mut EDataCont<T>, a_warnings: &mut ADataWarnings) {
    let mut seen_pks = RSet::new();
    let removed = e_cont
        .data
        .extract_if(.., |v| {
            let pk = v.get_pk();
            let is_seen = seen_pks.contains(&pk);
            if !is_seen {
                seen_pks.insert(pk);
            }
            is_seen
        })
        .count();
    if removed > 0 {
        let warning = format!("cleaned up {} PK duplicates for {}", removed, T::lib_get_name());
        a_warnings.pk_duplicates.push(warning);
    }
}
