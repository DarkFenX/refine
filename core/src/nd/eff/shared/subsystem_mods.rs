use crate::ad;

pub(in crate::nd::eff) fn make_modifier(
    src_attr_id: ad::AAttrId,
    affectee_attr_id: ad::AAttrId,
) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: src_attr_id,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id,
    }
}
