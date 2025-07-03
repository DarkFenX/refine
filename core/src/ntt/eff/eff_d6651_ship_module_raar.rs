use crate::{
    ac, ad, ec,
    ntt::{NttEffect, NttEffectRt, eff::shared::rep_amount::get_remote_armor_rep_amount},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::SHIP_MODULE_RAAR),
        aid: ac::effects::SHIP_MODULE_RAAR,
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        rt: NttEffectRt {
            get_remote_armor_rep_amount: Some(get_remote_armor_rep_amount),
            ..
        },
        ..
    }
}
