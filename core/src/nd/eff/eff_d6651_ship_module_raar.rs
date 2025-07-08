use crate::{
    ac, ec,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_restricted_s2s},
            rep_amount::get_remote_armor_rep_amount,
        },
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::SHIP_MODULE_RAAR),
        aid: ac::effects::SHIP_MODULE_RAAR,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::ChargeRate)),
            get_proj_mult: Some(get_proj_mult_normal_restricted_s2s),
            get_remote_armor_rep_amount: Some(get_remote_armor_rep_amount),
            ..
        },
        ..
    }
}
