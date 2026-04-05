use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItemListId,
    },
    ed::EEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_WEB;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_WEB;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::STASIS_WEBIFICATION_BURST,
                strength: AEffectBuffStrength::Attr(AAttrId::SPEED_FACTOR),
                duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
            }],
            ..
        }),
        modifier_proj: Some(NEffectProjGetter::AoeBurstRange),
        ..
    }
}
