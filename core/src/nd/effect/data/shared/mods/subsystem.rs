use crate::ad::{AAttrId, AEffectAffecteeFilter, AEffectLocation, AEffectModifier, AOp};

pub(in crate::nd::effect::data) fn mk_subsystem_mod(
    affector_attr_aid: AAttrId,
    affectee_attr_aid: AAttrId,
) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: affector_attr_aid,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: affectee_attr_aid,
    }
}
