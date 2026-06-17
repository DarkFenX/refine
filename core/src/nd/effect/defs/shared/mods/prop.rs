use crate::ad::{AAttrId, AEffectAffecteeFilter, AEffectLocation, AEffectModStrength, AEffectModifier, AOp, AValue};

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

pub(in crate::nd::effect::defs) fn mk_mjd_mods() -> impl ExactSizeIterator<Item = AEffectModifier> {
    // Besides blowing signature, MJD disables pretty much everything, except for tether (which is
    // still disabled for MJFGs due to aggro)
    [
        // Signature penalty
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::SIG_RADIUS_BONUS_PERCENT),
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::SIG_RADIUS,
        },
        // Disable warping
        AEffectModifier {
            strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::DISALLOW_WARPING,
        },
        // Disable gate jumping and jump drive
        AEffectModifier {
            strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::DISALLOW_DRIVE_JUMPING,
        },
        // Disable docking to stations, docking to citadels
        AEffectModifier {
            strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::DISALLOW_DOCKING,
        },
    ]
    .into_iter()
}
