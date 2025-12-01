use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc},
};

const E_EFFECT_ID: EEffectId = ec::effects::MOD_BONUS_WARFARE_LINK_SHIELD;
const A_EFFECT_ID: AEffectId = ac::effects::MOD_BONUS_WARFARE_LINK_SHIELD;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            default_attrs: Some(AEffectBuffScope::Fleet(ac::itemlists::SHIPS)),
            ..
        }),
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: false,
            }),
            ..
        },
        ..
    }
}
