use crate::{
    ac, ad, ec,
    ntt::{NttEffect, NttEffectHc, eff::shared::rep_amount::get_local_armor_rep_amount},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::FUELED_ARMOR_REPAIR),
        aid: ac::effects::FUELED_ARMOR_REPAIR,
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        hc: NttEffectHc {
            get_local_armor_rep_amount: Some(get_local_armor_rep_amount),
            ..
        },
        ..
    }
}
