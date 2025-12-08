use crate::{
    ad::{AAttrId, AEffectAffecteeFilter, AEffectModifier, AOp},
    rd::RAttrKey,
    util::RMap,
};

pub(crate) struct REffectModifier {
    pub(crate) affector_attr_key: RAttrKey,
    pub(crate) op: AOp,
    pub(crate) affectee_filter: AEffectAffecteeFilter,
    pub(crate) affectee_attr_key: RAttrKey,
}
impl REffectModifier {
    pub(in crate::rd::data::effect) fn try_from_a_effect_mod(
        a_effect_mod: &AEffectModifier,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) -> Option<Self> {
        let affector_attr_key = *attr_id_key_map.get(&a_effect_mod.affector_attr_id)?;
        let affectee_attr_key = *attr_id_key_map.get(&a_effect_mod.affectee_attr_id)?;
        Some(Self {
            affector_attr_key,
            op: a_effect_mod.op,
            affectee_filter: a_effect_mod.affectee_filter,
            affectee_attr_key,
        })
    }
}
