use std::collections::HashSet;

use crate::{
    adg::{rels::Pk, GData},
    defs::{ReeFloat, ReeInt},
    ed::EItemAttr,
};

pub(in crate::adg) fn normalize(g_data: &mut GData) {
    move_basic_attrs(g_data);
}

fn move_basic_attrs(g_data: &mut GData) {
    let mut seen_pks = HashSet::new();
    for item_attr in g_data.item_attrs.iter() {
        let pk = item_attr.get_pk();
        seen_pks.insert(pk);
    }
    for item in g_data.items.iter() {
        move_basic_attr(item.id, 38, item.capacity, &mut g_data.item_attrs, &seen_pks);
        move_basic_attr(item.id, 4, item.mass, &mut g_data.item_attrs, &seen_pks);
        move_basic_attr(item.id, 162, item.radius, &mut g_data.item_attrs, &seen_pks);
        move_basic_attr(item.id, 161, item.volume, &mut g_data.item_attrs, &seen_pks);
    }
}

fn move_basic_attr(
    item_id: ReeInt,
    attr_id: ReeInt,
    basic_value: ReeFloat,
    g_data_item_attrs: &mut Vec<EItemAttr>,
    seen_pks: &HashSet<Vec<ReeInt>>,
) {
    let item_attr = EItemAttr::new(item_id, attr_id, basic_value);
    let pk = item_attr.get_pk();
    if !seen_pks.contains(&pk) {
        g_data_item_attrs.push(item_attr)
    }
}
