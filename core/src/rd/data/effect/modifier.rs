use crate::{
    ad::{AAttrId, AEffectAffecteeFilter, AEffectModifier, AOp},
    rd::RAttrId,
    util::RMap,
};

pub(crate) struct REffectModifier {
    pub(crate) affector_attr_rid: RAttrId,
    pub(crate) op: AOp,
    pub(crate) affectee_filter: AEffectAffecteeFilter,
    pub(crate) affectee_attr_rid: RAttrId,
}
impl REffectModifier {
    pub(in crate::rd::data::effect) fn try_from_a_effect_mod(
        a_effect_mod: &AEffectModifier,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        let affector_attr_rid = *attr_aid_rid_map.get(&a_effect_mod.affector_attr_id)?;
        let affectee_attr_rid = *attr_aid_rid_map.get(&a_effect_mod.affectee_attr_id)?;
        Some(Self {
            affector_attr_rid,
            op: a_effect_mod.op,
            affectee_filter: a_effect_mod.affectee_filter,
            affectee_attr_rid,
        })
    }
}
