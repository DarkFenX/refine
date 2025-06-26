use crate::{ac, ad};

pub(in crate::ntt::eff) fn mk_mass_mod() -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: ac::attrs::MASS_ADDITION,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::MASS,
    }
}

pub(in crate::ntt::eff) fn mk_sig_mod() -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::SIG_RADIUS,
    }
}
