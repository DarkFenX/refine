use crate::{ac, ad::AEffectId, rd::REffectKey, util::RMap};

#[derive(Clone)]
pub(crate) struct REffectConsts {
    pub(crate) adaptive_armor_hardener: Option<REffectKey>,
    pub(crate) hi_power: Option<REffectKey>,
    pub(crate) lo_power: Option<REffectKey>,
    pub(crate) med_power: Option<REffectKey>,
    pub(crate) online: Option<REffectKey>,
    pub(crate) rig_slot: Option<REffectKey>,
    pub(crate) service_slot: Option<REffectKey>,
}
impl REffectConsts {
    pub(in crate::rd) fn new(effect_id_key_map: &RMap<AEffectId, REffectKey>) -> Self {
        Self {
            adaptive_armor_hardener: effect_id_key_map.get(&ac::effects::ADAPTIVE_ARMOR_HARDENER).copied(),
            hi_power: effect_id_key_map.get(&ac::effects::HI_POWER).copied(),
            lo_power: effect_id_key_map.get(&ac::effects::LO_POWER).copied(),
            med_power: effect_id_key_map.get(&ac::effects::MED_POWER).copied(),
            online: effect_id_key_map.get(&ac::effects::ONLINE).copied(),
            rig_slot: effect_id_key_map.get(&ac::effects::RIG_SLOT).copied(),
            service_slot: effect_id_key_map.get(&ac::effects::SERVICE_SLOT).copied(),
        }
    }
}
