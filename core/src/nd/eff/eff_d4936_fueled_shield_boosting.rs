use crate::{
    ac, ec,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc, eff::shared::rep_amount::get_local_shield_rep_amount},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::FUELED_SHIELD_BOOSTING),
        aid: ac::effects::FUELED_SHIELD_BOOSTING,
        hc: NEffectHc {
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::ChargeRate)),
            get_local_shield_rep_amount: Some(get_local_shield_rep_amount),
            ..
        },
        ..
    }
}
