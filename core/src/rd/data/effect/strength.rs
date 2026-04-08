use crate::{
    ad::{AAttrId, AEffectModStrength},
    num::Value,
    rd::RAttrId,
    util::RMap,
};

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum REffectModStrength {
    Attr(RAttrId),
    Hardcoded(Value),
}
impl REffectModStrength {
    pub(crate) fn get_attr_rid(&self) -> Option<RAttrId> {
        match self {
            Self::Attr(attr_rid) => Some(*attr_rid),
            Self::Hardcoded(_) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectModStrength {
    pub(super) fn try_from_a_mod_strength(
        a_mod_strength: &AEffectModStrength,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        match a_mod_strength {
            AEffectModStrength::Attr(attr_id) => Some(Self::Attr(*attr_aid_rid_map.get(attr_id)?)),
            AEffectModStrength::Hardcoded(val) => Some(Self::Hardcoded(Value::from_a_value(*val))),
        }
    }
}
