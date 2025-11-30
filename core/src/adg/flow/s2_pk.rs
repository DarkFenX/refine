//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use crate::{
    adg::rels::Pk,
    ed::{EData, EDataCont},
    util::{Named, RSet},
};

pub(in crate::adg) fn dedup_pks(e_data: &mut EData) {
    dedup_pks_vec(&mut e_data.items);
    dedup_pks_vec(&mut e_data.groups);
    dedup_pks_vec(&mut e_data.item_lists);
    dedup_pks_vec(&mut e_data.attrs);
    dedup_pks_vec(&mut e_data.item_attrs);
    dedup_pks_vec(&mut e_data.effects);
    dedup_pks_vec(&mut e_data.item_effects);
    dedup_pks_vec(&mut e_data.abils);
    dedup_pks_vec(&mut e_data.item_abils);
    dedup_pks_vec(&mut e_data.buffs);
    dedup_pks_vec(&mut e_data.space_comps);
    dedup_pks_vec(&mut e_data.item_srqs);
    dedup_pks_vec(&mut e_data.muta_items);
    dedup_pks_vec(&mut e_data.muta_attrs);
}

fn dedup_pks_vec<T: Pk + Named>(cont: &mut EDataCont<T>) {
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
        let msg = format!("cleaned up {} PK duplicates for {}", removed, T::get_name());
        tracing::warn!("{msg}");
    }
}
