use crate::{
    ac, ad, ec,
    ntt::{NttEffect, NttEffectRt, eff::shared::rep_amount::get_remote_shield_rep_amount},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::SHIP_MODULE_RASB),
        aid: ac::effects::SHIP_MODULE_RASB,
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        rt: NttEffectRt {
            get_remote_shield_rep_amount: Some(get_remote_shield_rep_amount),
            ..
        },
        ..
    }
}
