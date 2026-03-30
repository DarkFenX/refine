use crate::{ad::AAttrId, nd::NEffectResist, rd::RAttrId, util::RMap};

#[derive(Copy, Clone)]
pub(crate) enum REffectResist {
    // On-effect reference to resist attr ID, or, if it is not defined, on-item reference from the
    // standard remoteResistanceID attribute
    Standard,
    // Defines attribute whose value will have reference to resistance attribute ID
    AttrRef(RAttrId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectResist {
    pub(in crate::rd::data::effect) fn try_from_n_effect_resist(
        n_effect_resist: &NEffectResist,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(match n_effect_resist {
            NEffectResist::Standard => Self::Standard,
            NEffectResist::AttrRef(attr_aid) => Self::AttrRef(*attr_aid_rid_map.get(attr_aid)?),
        })
    }
}
