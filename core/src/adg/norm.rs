use crate::{
    adg::{
        rels::{KeyPart, Pk},
        GData,
    },
    defs::{AttrVal, EAttrId, EItemId},
    ed::EItemAttr,
    util::StSet,
};

pub(in crate::adg) fn normalize(g_data: &mut GData) {
    move_basic_attrs(g_data);
}

fn move_basic_attrs(g_data: &mut GData) {
    let mut seen_pks = StSet::new();
    for item_attr in g_data.item_attrs.iter() {
        let pk = item_attr.get_pk();
        seen_pks.insert(pk);
    }
    let attr_ids = g_data.attrs.iter().map(|v| v.id).collect();
    for item in g_data.items.iter() {
        move_basic_attr(item.id, 38, item.capacity, &mut g_data.item_attrs, &attr_ids, &seen_pks);
        move_basic_attr(item.id, 4, item.mass, &mut g_data.item_attrs, &attr_ids, &seen_pks);
        move_basic_attr(item.id, 162, item.radius, &mut g_data.item_attrs, &attr_ids, &seen_pks);
        move_basic_attr(item.id, 161, item.volume, &mut g_data.item_attrs, &attr_ids, &seen_pks);
    }
}

fn move_basic_attr(
    item_id: EItemId,
    attr_id: EAttrId,
    basic_value: AttrVal,
    g_data_item_attrs: &mut Vec<EItemAttr>,
    attr_ids: &StSet<EAttrId>,
    seen_pks: &StSet<Vec<KeyPart>>,
) {
    // Shouldn't be useful on actual data, but causes lots of broken relations when running tests
    if !attr_ids.contains(&attr_id) {
        return;
    }
    let item_attr = EItemAttr::new(item_id, attr_id, basic_value);
    let pk = item_attr.get_pk();
    if !seen_pks.contains(&pk) {
        g_data_item_attrs.push(item_attr)
    }
}
