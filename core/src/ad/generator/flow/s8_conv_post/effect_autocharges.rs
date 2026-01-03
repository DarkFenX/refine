use crate::{
    ad::{AAttrId, AData, AItemId},
    nd::{N_EFFECT_MAP, NEffectChargeLoc},
};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_effect_autocharges(a_data: &mut AData) {
    for a_item in a_data.items.values_mut() {
        for (a_effect_id, a_effect_data) in a_item.effect_datas.iter_mut() {
            if let Some(n_effect) = N_EFFECT_MAP.get(a_effect_id)
                && let Some(n_charge) = &n_effect.charge
                && let Some(ac_attr_aid) = n_charge.location.get_autocharge_attr_aid()
                && let Some(&attr_val) = a_item.attrs.get(&ac_attr_aid)
            {
                let ac_item_aid = AItemId::new_of64(attr_val.into_inner());
                if ac_item_aid != AItemId::new(0) {
                    a_effect_data.autocharge = Some(ac_item_aid)
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
