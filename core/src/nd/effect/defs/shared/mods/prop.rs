use crate::ad::{AAttrId, AEffectAffecteeFilter, AEffectLocation, AEffectModStrength, AEffectModifier, AOp};

pub(in crate::nd::effect::defs) fn mk_prop_mass_mod() -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::MASS_ADDITION),
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::MASS,
    }
}
pub(in crate::nd::effect::defs) fn mk_mwd_sig_mod() -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::SIG_RADIUS_BONUS),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::SIG_RADIUS,
    }
}
pub(in crate::nd::effect::defs) fn mk_mjd_sig_mod() -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::SIG_RADIUS_BONUS_PERCENT),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::SIG_RADIUS,
    }
}
