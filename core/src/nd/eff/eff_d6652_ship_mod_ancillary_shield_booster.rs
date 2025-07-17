use crate::{
    ac, ec,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_restricted_s2s},
            rep_amount::get_remote_shield_rep_amount,
        },
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER),
        aid: ac::effects::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::ChargeRate {
                can_run_uncharged: true,
            })),
            get_proj_mult: Some(get_proj_mult_normal_restricted_s2s),
            get_remote_shield_rep_amount: Some(get_remote_shield_rep_amount),
            ..
        },
        ..
    }
}
