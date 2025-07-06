use crate::{
    ac, ad, ec,
    ntt::{NttEffect, NttEffectHc, eff::shared::rep_amount::get_local_shield_rep_amount},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::FUELED_SHIELD_BOOSTING),
        aid: ac::effects::FUELED_SHIELD_BOOSTING,
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        hc: NttEffectHc {
            get_local_shield_rep_amount: Some(get_local_shield_rep_amount),
            ..
        },
        ..
    }
}
