use crate::{
    ad::{AAttrId, AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId, AItemListId},
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::MOD_TITAN_EFFECT_GENERATOR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(AAttrId::BUFF_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS),
            }),
            ..
        }),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: NEffectProjGetter::GenericRangeSimpleSts,
            ..
        }),
        ..
    }
}
