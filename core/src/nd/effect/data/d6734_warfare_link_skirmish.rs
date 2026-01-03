use crate::{
    ac,
    ad::{AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc},
};

const EFFECT_EID: EEffectId = ec::effects::MOD_BONUS_WARFARE_LINK_SKIRMISH;
const EFFECT_AID: AEffectId = ac::effects::MOD_BONUS_WARFARE_LINK_SKIRMISH;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(ac::attrs::BUFF_DURATION),
                scope: AEffectBuffScope::Fleet(ac::itemlists::SHIPS),
            }),
            ..
        }),
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        ..
    }
}
