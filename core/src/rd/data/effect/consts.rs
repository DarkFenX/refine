use crate::{ac, ad::AEffectId, rd::REffectId, util::RMap};

#[derive(Clone)]
pub(crate) struct REffectConsts {
    pub(crate) adaptive_armor_hardener: Option<REffectId>,
    pub(crate) hi_power: Option<REffectId>,
    pub(crate) lo_power: Option<REffectId>,
    pub(crate) med_power: Option<REffectId>,
    pub(crate) online: Option<REffectId>,
    pub(crate) rig_slot: Option<REffectId>,
    pub(crate) service_slot: Option<REffectId>,
}
impl REffectConsts {
    pub(in crate::rd) fn new(effect_aid_rid_map: &RMap<AEffectId, REffectId>) -> Self {
        Self {
            adaptive_armor_hardener: effect_aid_rid_map.get(&ac::effects::ADAPTIVE_ARMOR_HARDENER).copied(),
            hi_power: effect_aid_rid_map.get(&ac::effects::HI_POWER).copied(),
            lo_power: effect_aid_rid_map.get(&ac::effects::LO_POWER).copied(),
            med_power: effect_aid_rid_map.get(&ac::effects::MED_POWER).copied(),
            online: effect_aid_rid_map.get(&ac::effects::ONLINE).copied(),
            rig_slot: effect_aid_rid_map.get(&ac::effects::RIG_SLOT).copied(),
            service_slot: effect_aid_rid_map.get(&ac::effects::SERVICE_SLOT).copied(),
        }
    }
}
