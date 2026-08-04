use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength,
    },
    nd::{NEffect, NEffectTime},
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_BREACHER_POD_DAMAGE_CONTROL;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::BPDC_DMG_RECEIVED_PERC,
                strength: AEffectModStrength::Attr(AAttrId::BREACHER_POD_ACTIVATED_DMG_RECEIVED_PERCENT),
                duration: AEffectBuffDuration::Effect,
                scope: AEffectBuffScope::Carrier,
            }],
            ..
        }),
        kills_item: Some(NEffectTime::CycleEnd),
        ..
    }
}
