use crate::{
    ad::{AAttrId, ADataGenerator},
    nd::{N_EFFECT_MAP, NEffectChargeLoc},
};

impl ADataGenerator {
    pub(super) fn fill_effect_autocharges(&mut self) {
        for a_item in self.a_data.items.data.values_mut() {
            for a_item_effect in a_item.effects.iter_mut() {
                if let Some(n_effect) = N_EFFECT_MAP.get(&a_item_effect.id)
                    && let Some(n_charge) = &n_effect.charge
                    && let Some(ac_attr_aid) = n_charge.location.get_autocharge_attr_aid()
                {
                    a_item_effect.data.autocharge_attr_id = Some(ac_attr_aid)
                }
            }
        }
    }
}

impl NEffectChargeLoc {
    fn get_autocharge_attr_aid(&self) -> Option<AAttrId> {
        match self {
            Self::Loaded(..) => None,
            Self::Autocharge(attr_aid) => Some(*attr_aid),
            Self::TargetAttack(attr_aid) => Some(*attr_aid),
        }
    }
}
