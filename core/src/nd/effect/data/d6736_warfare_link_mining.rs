use crate::{
    ac,
    ad::{AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc},
};

const E_EFFECT_ID: EEffectId = ec::effects::MOD_BONUS_WARFARE_LINK_MINING;
const A_EFFECT_ID: AEffectId = ac::effects::MOD_BONUS_WARFARE_LINK_MINING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(ac::attrs::BUFF_DURATION),
                scope: AEffectBuffScope::Fleet(ac::itemlists::SHIPS),
            }),
            ..
        }),
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                can_run_uncharged: false,
            }),
            activates_charge: false,
        }),
        ..
    }
}
