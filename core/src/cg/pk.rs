//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use std::collections::HashSet;

use crate::util::Named;

use super::data::{Data, Pk};

fn dedup_pks_vec<T: Pk + Named>(vec: &mut Vec<T>, warns: &mut Vec<String>) {
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
        log::warn!("{}", &msg);
        warns.push(msg);
    }
}

pub(super) fn dedup_pks(cg_data: &mut Data, warns: &mut Vec<String>) {
    dedup_pks_vec(&mut cg_data.items, warns);
    dedup_pks_vec(&mut cg_data.groups, warns);
    dedup_pks_vec(&mut cg_data.attrs, warns);
    dedup_pks_vec(&mut cg_data.item_attrs, warns);
    dedup_pks_vec(&mut cg_data.effects, warns);
    dedup_pks_vec(&mut cg_data.item_effects, warns);
    dedup_pks_vec(&mut cg_data.abils, warns);
    dedup_pks_vec(&mut cg_data.item_abils, warns);
    dedup_pks_vec(&mut cg_data.buffs, warns);
    dedup_pks_vec(&mut cg_data.item_srqs, warns);
    dedup_pks_vec(&mut cg_data.muta_items, warns);
    dedup_pks_vec(&mut cg_data.muta_attrs, warns);
}
