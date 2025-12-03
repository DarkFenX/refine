use crate::ad::{AAttrId, AEffectAffecteeFilter, AEffectLocation, AEffectModifier, AOp};

pub(in crate::nd::effect) fn mk_subsystem_mod(src_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: src_attr_id,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id,
    }
}
