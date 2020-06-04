use std::collections::HashMap;

use itertools::Itertools;

use crate::ct;

use super::Data;

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(data: &Data) {
    conv_attrs(data);
    conv_mutas(data);
}

fn conv_attrs(data: &Data) -> Vec<ct::Attr> {
    data.attrs
        .iter()
        .map(|v| ct::Attr::new(v.id, !v.stackable, v.high_is_good, v.default_value, v.max_attr_id))
        .collect()
}

fn conv_mutas(data: &Data) -> Vec<ct::Muta> {
    let mut composed = HashMap::new();
    for item_data in data.muta_items.iter() {
        let muta = composed
            .entry(item_data.muta_id)
            .or_insert_with(|| ct::Muta::new(item_data.muta_id));
        muta.item_map.insert(item_data.in_item_id, item_data.out_item_id);
    }
    for attr_data in data.muta_attrs.iter() {
        // We are interested in attribute modifiers only for mutaplasmids which have in-out item
        // definitions
        if let Some(muta) = composed.get_mut(&attr_data.muta_id) {
            muta.attr_mods.insert(
                attr_data.attr_id,
                ct::MutaAttrRange::new(attr_data.min_attr_mult, attr_data.max_attr_mult),
            );
        }
    }
    composed.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id).collect()
}
