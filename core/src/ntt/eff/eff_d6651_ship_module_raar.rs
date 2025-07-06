use crate::{
    ac, ad, ec,
    ntt::{
        NttEffect, NttEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_restricted_s2s},
            rep_amount::get_remote_armor_rep_amount,
        },
    },
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::SHIP_MODULE_RAAR),
        aid: ac::effects::SHIP_MODULE_RAAR,
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NttEffectHc {
            get_proj_mult: Some(get_proj_mult_normal_restricted_s2s),
            get_remote_armor_rep_amount: Some(get_remote_armor_rep_amount),
            ..
        },
        ..
    }
}
