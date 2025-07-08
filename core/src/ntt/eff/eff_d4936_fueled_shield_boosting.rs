use crate::{
    ac, ec,
    ntt::{
        NttEffect, NttEffectCharge, NttEffectChargeDepl, NttEffectHc,
        eff::shared::rep_amount::get_local_shield_rep_amount,
    },
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::FUELED_SHIELD_BOOSTING),
        aid: ac::effects::FUELED_SHIELD_BOOSTING,
        hc: NttEffectHc {
            charge: Some(NttEffectCharge::Loaded(NttEffectChargeDepl::ChargeRate)),
            get_local_shield_rep_amount: Some(get_local_shield_rep_amount),
            ..
        },
        ..
    }
}
