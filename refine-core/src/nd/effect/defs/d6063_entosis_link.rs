use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::ENTOSIS_LINK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            // Entosis doesn't show buff icon, but seems to use some way to transfer scram strength
            // to parent ship. The easiest way to replicate that is to do it via self-debuff.
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::WARP_PENALTY,
                strength: AEffectModStrength::Attr(AAttrId::SIEGE_MODE_WARP_STATUS),
                duration: AEffectBuffDuration::Effect,
                scope: AEffectBuffScope::Carrier,
            }],
            ..
        }),
        ..
    }
}
