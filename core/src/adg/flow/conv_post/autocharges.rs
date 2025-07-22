use crate::{ad, nd};

pub(in crate::adg::flow::conv_post) fn fill_autocharges(a_data: &mut ad::AData) {
    for a_item in a_data.items.values_mut() {
        for (a_effect_id, a_effect_data) in a_item.effect_datas.iter_mut() {
            if let Some(n_effect) = nd::N_EFFECT_MAP.get(a_effect_id)
                && let Some(n_charge) = n_effect.hc.charge
                && let nd::NEffectChargeLoc::Autocharge(ac_attr_id) = n_charge.location
                && let Some(attr_val) = a_item.attrs.get(&ac_attr_id)
            {
                a_effect_data.autocharge = Some(attr_val.round() as ad::AItemId)
            }
        }
    }
}
