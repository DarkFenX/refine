use crate::{
    ad::{AAttrId, AEffect},
    dbg::DebugResult,
    nd::NEffectResist,
    rd::RAttrId,
    ud::UData,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) enum REffectResist {
    // Resistance attribute ID
    Attr(RAttrId),
    // Value of this projector attribute references actual resistance attribute ID
    AttrRef(RAttrId),
    // Value of remoteResistanceID projector attribute references actual resistance attribute ID.
    // Special-cased for optimization purposes.
    RemoteResistance,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectResist {
    pub(in crate::rd::data::effect) fn try_from_n_effect_resist(
        n_effect_resist: &NEffectResist,
        a_effect: &AEffect,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(match n_effect_resist {
            NEffectResist::Standard => match a_effect.resist_attr_id.as_ref() {
                Some(attr_aid) => Self::Attr(*attr_aid_rid_map.get(attr_aid)?),
                None => Self::RemoteResistance,
            },
            NEffectResist::Attr(attr_aid) => Self::Attr(*attr_aid_rid_map.get(attr_aid)?),
            NEffectResist::AttrRef(attr_aid) => Self::AttrRef(*attr_aid_rid_map.get(attr_aid)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectResist {
    pub(in crate::rd) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        match self {
            REffectResist::Attr(attr_rid) => attr_rid.consistency_check(u_data)?,
            REffectResist::AttrRef(attr_rid) => attr_rid.consistency_check(u_data)?,
            REffectResist::RemoteResistance => (),
        }
        Ok(())
    }
}
