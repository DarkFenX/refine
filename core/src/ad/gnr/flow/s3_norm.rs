use crate::{
    ad::gnr::rels::{KeyPart, Pk},
    ed::{EAttrId, EAttrVal, EData, EDataCont, EItemAttr, EItemId},
    util::RSet,
};

pub(in crate::ad::gnr) fn normalize(e_data: &mut EData) {
    move_basic_attrs(e_data);
}

fn move_basic_attrs(e_data: &mut EData) {
    let mut seen_pks = RSet::new();
    for item_attr in e_data.item_attrs.data.iter() {
        let pk = item_attr.get_pk();
        seen_pks.insert(pk);
    }
    let attr_ids = e_data.attrs.data.iter().map(|v| v.id).collect();
    for item in e_data.items.data.iter() {
        move_basic_attr(item.id, 38, item.capacity, &mut e_data.item_attrs, &attr_ids, &seen_pks);
        move_basic_attr(item.id, 4, item.mass, &mut e_data.item_attrs, &attr_ids, &seen_pks);
        move_basic_attr(item.id, 162, item.radius, &mut e_data.item_attrs, &attr_ids, &seen_pks);
        move_basic_attr(item.id, 161, item.volume, &mut e_data.item_attrs, &attr_ids, &seen_pks);
    }
}

fn move_basic_attr(
    item_id: EItemId,
    attr_id: EAttrId,
    basic_value: EAttrVal,
    e_data_item_attrs: &mut EDataCont<EItemAttr>,
    attr_ids: &RSet<EAttrId>,
    seen_pks: &RSet<Vec<KeyPart>>,
) {
    // Shouldn't be useful on actual data, but causes lots of broken relations when running tests
    if !attr_ids.contains(&attr_id) {
        return;
    }
    let item_attr = EItemAttr {
        item_id,
        attr_id,
        value: basic_value,
    };
    let pk = item_attr.get_pk();
    if !seen_pks.contains(&pk) {
        e_data_item_attrs.data.push(item_attr)
    }
}
