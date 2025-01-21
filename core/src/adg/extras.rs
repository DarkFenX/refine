use crate::ad;

pub(in crate::adg) fn fill_extra_data(a_data: &mut ad::AData) {
    for a_item in a_data.items.iter_mut() {
        a_item
            .extras
            .update(a_item.grp_id, a_item.cat_id, &a_item.attr_vals, &a_item.effect_datas)
    }
}
