use crate::{
    ac,
    ad::{AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffInfo, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc},
};

const E_EFFECT_ID: EEffectId = ec::effects::MOD_BONUS_WARFARE_LINK_INFO;
const A_EFFECT_ID: AEffectId = ac::effects::MOD_BONUS_WARFARE_LINK_INFO;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(ac::attrs::BUFF_DURATION),
                scope: AEffectBuffScope::Fleet(ac::itemlists::SHIPS),
            }),
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
