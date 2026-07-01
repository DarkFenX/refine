//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use crate::{
    ad::{ADataGenerator, generator::rels::Pk},
    ed::EDataCont,
    util::{LibNamed, RSet},
};

impl ADataGenerator {
    pub(in crate::ad::generator) fn dedup_pks(&mut self) {
        dedup_pks_vec(&mut self.e_data.items);
        dedup_pks_vec(&mut self.e_data.groups);
        dedup_pks_vec(&mut self.e_data.item_lists);
        dedup_pks_vec(&mut self.e_data.attrs);
        dedup_pks_vec(&mut self.e_data.item_attrs);
        dedup_pks_vec(&mut self.e_data.effects);
        dedup_pks_vec(&mut self.e_data.item_effects);
        dedup_pks_vec(&mut self.e_data.abils);
        dedup_pks_vec(&mut self.e_data.item_abils);
        dedup_pks_vec(&mut self.e_data.buffs);
        dedup_pks_vec(&mut self.e_data.space_comps);
        dedup_pks_vec(&mut self.e_data.item_srqs);
        dedup_pks_vec(&mut self.e_data.muta_items);
        dedup_pks_vec(&mut self.e_data.muta_attrs);
    }
}

fn dedup_pks_vec<T: Pk + LibNamed>(cont: &mut EDataCont<T>) {
    let mut seen_pks = RSet::new();
    let removed = cont
        .data
        .extract_if(.., |v| {
            let pk = v.get_pk();
            if seen_pks.contains(&pk) {
                true
            } else {
                seen_pks.insert(pk);
                false
            }
        })
        .count();
    if removed > 0 {
        let msg = format!("cleaned up {} PK duplicates for {}", removed, T::lib_get_name());
        tracing::warn!("{msg}");
    }
}
