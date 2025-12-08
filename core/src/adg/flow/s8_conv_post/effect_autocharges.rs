use crate::{
    ad::{AData, AItemId},
    nd::N_EFFECT_MAP,
};

pub(in crate::adg::flow::s8_conv_post) fn fill_effect_autocharges(a_data: &mut AData) {
    for a_item in a_data.items.values_mut() {
        for (a_effect_id, a_effect_data) in a_item.effect_datas.iter_mut() {
            if let Some(n_effect) = N_EFFECT_MAP.get(a_effect_id)
                && let Some(n_charge) = &n_effect.hc.charge
                && let Some(ac_attr_id) = n_charge.location.get_autocharge_attr_id()
                && let Some(&attr_val) = a_item.attrs.get(&ac_attr_id)
            {
                match attr_val.round() as AItemId {
                    0 => (),
                    a_item_id => a_effect_data.autocharge = Some(a_item_id),
                }
            }
        }
    }
}
