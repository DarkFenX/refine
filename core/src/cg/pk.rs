//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use std::collections::HashSet;

use crate::util::Named;

use super::data::{Data, Pk};

fn dedup_pks_vec<T>(vec: &mut Vec<T>, errs: &mut Vec<String>)
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
        errs.push(msg);
    }
}

pub(super) fn dedup_pks(data: &mut Data, errs: &mut Vec<String>) {
    dedup_pks_vec(&mut data.items, errs);
    dedup_pks_vec(&mut data.groups, errs);
    dedup_pks_vec(&mut data.attrs, errs);
    dedup_pks_vec(&mut data.item_attrs, errs);
    dedup_pks_vec(&mut data.effects, errs);
    dedup_pks_vec(&mut data.item_effects, errs);
    dedup_pks_vec(&mut data.abils, errs);
    dedup_pks_vec(&mut data.item_abils, errs);
    dedup_pks_vec(&mut data.buffs, errs);
    dedup_pks_vec(&mut data.item_srqs, errs);
    dedup_pks_vec(&mut data.muta_items, errs);
    dedup_pks_vec(&mut data.muta_attrs, errs);
}
