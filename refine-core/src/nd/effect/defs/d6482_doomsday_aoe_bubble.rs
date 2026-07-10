use super::shared::mk_bubble_buff;
use crate::{
    ad::{AAttrId, AEffectBuff, AEffectBuffDuration, AEffectId},
    nd::{NEffect, NEffectDuration, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_BUBBLE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![mk_bubble_buff(AEffectBuffDuration::AttrMs(
                AAttrId::DOOMSDAY_AOE_DURATION,
            ))],
            ..
        }),
        disallows_cloak: Some(NEffectDuration::Effect),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::AoeBurstRange),
            ..
        }),
        ..
    }
}
