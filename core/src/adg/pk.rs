//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use std::collections::HashSet;

use crate::{
    adg::{rels::Pk, GData},
    util::Named,
};

fn dedup_pks_vec<T: Pk + Named>(vec: &mut Vec<T>) {
    let mut seen_pks = HashSet::new();
    let removed = vec
        .drain_filter(|v| {
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
        log::warn!("{msg}");
    }
}

pub(in crate::adg) fn dedup_pks(g_data: &mut GData) {
    dedup_pks_vec(&mut g_data.items);
    dedup_pks_vec(&mut g_data.groups);
    dedup_pks_vec(&mut g_data.attrs);
    dedup_pks_vec(&mut g_data.item_attrs);
    dedup_pks_vec(&mut g_data.effects);
    dedup_pks_vec(&mut g_data.item_effects);
    dedup_pks_vec(&mut g_data.abils);
    dedup_pks_vec(&mut g_data.item_abils);
    dedup_pks_vec(&mut g_data.buffs);
    dedup_pks_vec(&mut g_data.item_srqs);
    dedup_pks_vec(&mut g_data.muta_items);
    dedup_pks_vec(&mut g_data.muta_attrs);
}
