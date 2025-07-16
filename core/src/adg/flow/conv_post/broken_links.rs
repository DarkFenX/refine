use crate::ad;

pub(in crate::adg::flow::conv_post) fn clear_broken_links(a_data: &mut ad::AData) {
    clear_broken_item_effect_links(a_data);
}

// Guarantees that all the links to effect from item will yield an actual effect, so that code in
// the lib doesn't have to care much about it
fn clear_broken_item_effect_links(a_data: &mut ad::AData) {
    for a_item in a_data.items.values_mut() {
        a_item
            .effect_datas
            .retain(|a_effect_id, _| a_data.effects.contains_key(a_effect_id));
        if let Some(a_effect_id) = a_item.defeff_id
            && !a_data.effects.contains_key(&a_effect_id)
        {
            a_item.defeff_id = None;
        }
    }
}
