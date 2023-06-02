use std::collections::HashMap;

use itertools::Itertools;

use crate::adt;

use super::Data;

pub(super) fn conv_mutas(erg_data: &Data) -> Vec<adt::AMuta> {
    let mut composed = HashMap::new();
    for item_data in erg_data.muta_items.iter() {
        let muta = composed
            .entry(item_data.muta_id)
            .or_insert_with(|| adt::AMuta::new(item_data.muta_id));
        muta.item_map.insert(item_data.in_item_id, item_data.out_item_id);
    }
    for attr_data in erg_data.muta_attrs.iter() {
        // We are interested in attribute modifiers only for mutaplasmids which have in-out item
        // definitions
        if let Some(muta) = composed.get_mut(&attr_data.muta_id) {
            muta.attr_mods.insert(
                attr_data.attr_id,
                adt::AMutaAttrRange::new(attr_data.min_attr_mult, attr_data.max_attr_mult),
            );
        }
    }
    composed.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id).collect()
}
