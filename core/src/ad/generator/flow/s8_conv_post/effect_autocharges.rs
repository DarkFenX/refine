use crate::{
    ad::{AAttrId, AData, AItemId},
    nd::{N_EFFECT_MAP, NEffectChargeLoc},
};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_effect_autocharges(a_data: &mut AData) {
    for a_item in a_data.items.data.values_mut() {
        for a_item_effect in a_item.effects.iter_mut() {
            if let Some(n_effect) = N_EFFECT_MAP.get(&a_item_effect.id)
                && let Some(n_charge) = &n_effect.charge
                && let Some(ac_attr_aid) = n_charge.location.get_autocharge_attr_aid()
                && let Some(ac_a_item_attr) = a_item.attrs.get(&ac_attr_aid)
            {
                let ac_item_aid = AItemId::from_f64_rounded(ac_a_item_attr.value.into_f64());
                if ac_item_aid != AItemId::from_i32(0) {
                    a_item_effect.data.autocharge = Some(ac_item_aid)
                }
            }
        }
    }
}

impl NEffectChargeLoc {
    pub(in crate::ad::generator) fn get_autocharge_attr_aid(&self) -> Option<AAttrId> {
        match self {
            Self::Loaded(_) => None,
            Self::Autocharge(attr_aid) => Some(*attr_aid),
            Self::TargetAttack(attr_aid) => Some(*attr_aid),
        }
    }
}
