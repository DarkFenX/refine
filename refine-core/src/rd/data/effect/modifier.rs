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
        Some(Self {
            strength: REffectModStrength::try_from_a_mod_strength(&a_effect_mod.strength, attr_aid_rid_map)?,
            op: a_effect_mod.op,
            affectee_filter: a_effect_mod.affectee_filter,
            affectee_attr_rid: *attr_aid_rid_map.get(&a_effect_mod.affectee_attr_id)?,
        })
    }
}
