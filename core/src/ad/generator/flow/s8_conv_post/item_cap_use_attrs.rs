use crate::ad::AData;

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_cap_use_attr_ids(a_data: &mut AData) {
    for a_item in a_data.items.values_mut() {
        for effect_id in a_item.effect_datas.keys() {
            if let Some(a_effect) = a_data.effects.get(effect_id)
                && let Some(attr_id) = a_effect.discharge_attr_id
                && !a_item.cap_use_attr_ids.contains(&attr_id)
            {
                a_item.cap_use_attr_ids.push(attr_id);
            }
        }
    }
}
