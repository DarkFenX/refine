//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use std::collections::HashSet;

use crate::util::Named;

use super::data::{Data, Pk};

fn dedup_pks_vec<T>(vec: &mut Vec<T>, warns: &mut Vec<String>)
where
    T: Pk + Named,
{
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

pub(super) fn dedup_pks(data: &mut Data, warns: &mut Vec<String>) {
    dedup_pks_vec(&mut data.items, warns);
    dedup_pks_vec(&mut data.groups, warns);
    dedup_pks_vec(&mut data.attrs, warns);
    dedup_pks_vec(&mut data.item_attrs, warns);
    dedup_pks_vec(&mut data.effects, warns);
    dedup_pks_vec(&mut data.item_effects, warns);
    dedup_pks_vec(&mut data.abils, warns);
    dedup_pks_vec(&mut data.item_abils, warns);
    dedup_pks_vec(&mut data.buffs, warns);
    dedup_pks_vec(&mut data.item_srqs, warns);
    dedup_pks_vec(&mut data.muta_items, warns);
    dedup_pks_vec(&mut data.muta_attrs, warns);
}
