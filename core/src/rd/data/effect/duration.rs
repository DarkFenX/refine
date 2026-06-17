use crate::{ad::AAttrId, nd::NEffectDuration, rd::RAttrId, util::RMap};

pub(crate) enum REffectDuration {
    Effect,
    AttrMs(RAttrId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectDuration {
    pub(in crate::rd::data::effect) fn try_from_n_effect_duration(
        n_effect_duration: &NEffectDuration,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(match n_effect_duration {
            NEffectDuration::Effect => Self::Effect,
            NEffectDuration::AttrMs(attr_aid) => Self::AttrMs(*attr_aid_rid_map.get(attr_aid)?),
        })
    }
}
