use crate::ad::{AAttrId, AEffectAffecteeFilter, AEffectLocation, AEffectModStrength, AEffectModifier, AOp};

pub(in crate::nd::effect::defs) fn mk_disallow_assistance_mod() -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::DISALLOW_ASSISTANCE),
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::DISALLOW_ASSISTANCE,
    }
}
