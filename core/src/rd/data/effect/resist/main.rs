use crate::{ad::AAttrId, nd::NEffectResist, rd::RAttrId, util::RMap};

#[derive(Copy, Clone)]
pub(crate) enum REffectResist {
    Standard,
    Attr(RAttrId),
}
impl REffectResist {
    pub(in crate::rd::data::effect) fn try_from_n_effect_resist(
        n_effect_resist: &NEffectResist,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(match n_effect_resist {
            NEffectResist::Standard => Self::Standard,
            NEffectResist::Attr(attr_aid) => Self::Attr(*attr_aid_rid_map.get(&attr_aid)?),
        })
    }
}
