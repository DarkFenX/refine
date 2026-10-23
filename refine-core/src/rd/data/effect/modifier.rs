use crate::{
    ad::{AAttrId, AEffectAffecteeFilter, AEffectModifier, AOp},
    rd::{RAttrId, REffectModStrength},
    util::RMap,
};

pub(crate) struct REffectModifier {
    pub(crate) strength: REffectModStrength,
    pub(crate) op: AOp,
    pub(crate) affectee_filter: AEffectAffecteeFilter,
    pub(crate) affectee_attr_rid: RAttrId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectModifier {
    pub(in crate::rd::data::effect) fn try_from_a_effect_mod(
        a_effect_mod: &AEffectModifier,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        let AEffectModifier {
            strength,
            op,
            affectee_filter,
            affectee_attr_id,
        } = a_effect_mod;
        Some(Self {
            strength: REffectModStrength::try_from_a_mod_strength(strength, attr_aid_rid_map)?,
            op: *op,
            affectee_filter: *affectee_filter,
            affectee_attr_rid: *attr_aid_rid_map.get(affectee_attr_id)?,
        })
    }
}
