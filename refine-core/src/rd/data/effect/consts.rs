use crate::{ad::AEffectId, rd::REffectId, util::RMap};

#[derive(Clone)]
pub(crate) struct REffectConsts {
    pub(crate) adaptive_armor_hardener: Option<REffectId>,
    pub(in crate::rd::data) hi_power: Option<REffectId>,
    pub(in crate::rd::data) lo_power: Option<REffectId>,
    pub(in crate::rd::data) med_power: Option<REffectId>,
    pub(crate) online: Option<REffectId>,
    pub(in crate::rd::data) rig_slot: Option<REffectId>,
    pub(in crate::rd::data) service_slot: Option<REffectId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectConsts {
    pub(in crate::rd::data) fn from_id_map(effect_aid_rid_map: &RMap<AEffectId, REffectId>) -> Self {
        Self {
            adaptive_armor_hardener: effect_aid_rid_map.get(&AEffectId::ADAPTIVE_ARMOR_HARDENER).copied(),
            hi_power: effect_aid_rid_map.get(&AEffectId::HI_POWER).copied(),
            lo_power: effect_aid_rid_map.get(&AEffectId::LO_POWER).copied(),
            med_power: effect_aid_rid_map.get(&AEffectId::MED_POWER).copied(),
            online: effect_aid_rid_map.get(&AEffectId::ONLINE).copied(),
            rig_slot: effect_aid_rid_map.get(&AEffectId::RIG_SLOT).copied(),
            service_slot: effect_aid_rid_map.get(&AEffectId::SERVICE_SLOT).copied(),
        }
    }
}
