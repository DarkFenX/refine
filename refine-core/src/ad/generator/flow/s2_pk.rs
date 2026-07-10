//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use crate::{
    ad::{ADataGenerator, ADataWarnings, generator::rels::Pk},
    ed::EDataCont,
    util::{LibNamed, RSet},
};

impl ADataGenerator {
    pub(in crate::ad::generator) fn dedup_pks(&mut self) {
        dedup_pks_vec(&mut self.e_data.items, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.groups, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.item_lists, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.attrs, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.item_attrs, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.effects, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.item_effects, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.abils, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.item_abils, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.buffs, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.space_comps, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.item_srqs, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.muta_items, &mut self.a_data.warnings);
        dedup_pks_vec(&mut self.e_data.muta_attrs, &mut self.a_data.warnings);
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
