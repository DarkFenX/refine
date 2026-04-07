use crate::{ad::AData, nd::N_EFFECT_MAP};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_cloaks(a_data: &mut AData) {
    for a_item in a_data.items.data.values_mut() {
        for a_item_effect in a_item.effects.iter() {
            let n_effect = match N_EFFECT_MAP.get(&a_item_effect.id) {
                Some(n_effect) => n_effect,
                None => continue,
            };
            if n_effect.cloaks_carrier {
                a_item.is_cloak = true;
                break;
            }
        }
    }
}
