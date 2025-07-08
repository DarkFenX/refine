use crate::{
    ac, ec,
    ntt::{
        NttEffect, NttEffectCharge, NttEffectChargeDepl, NttEffectHc,
        eff::shared::rep_amount::get_local_armor_rep_amount,
    },
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::FUELED_ARMOR_REPAIR),
        aid: ac::effects::FUELED_ARMOR_REPAIR,
        hc: NttEffectHc {
            charge: Some(NttEffectCharge::Loaded(NttEffectChargeDepl::ChargeRate)),
            get_local_armor_rep_amount: Some(get_local_armor_rep_amount),
            ..
        },
        ..
    }
}
