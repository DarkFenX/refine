use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength, AItemListId,
    },
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_PAINT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::SIGNATURE_RADIUS_PENALTY,
                strength: AEffectModStrength::Attr(AAttrId::SIG_RADIUS_BONUS),
                duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
            }],
            ..
        }),
        modifier_proj: Some(NEffectProjGetter::AoeBurstRange),
        ..
    }
}
