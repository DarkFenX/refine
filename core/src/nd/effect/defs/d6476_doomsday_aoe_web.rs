use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength, AItemListId,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_WEB;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::STASIS_WEBIFICATION_BURST,
                strength: AEffectModStrength::Attr(AAttrId::SPEED_FACTOR),
                duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
            }],
            ..
        }),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: NEffectProjGetter::AoeBurstRange,
            ..
        }),
        ..
    }
}
