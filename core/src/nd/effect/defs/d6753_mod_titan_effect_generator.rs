use crate::{
    ad::{
        AAttrId, AEffect, AEffectAggroDuration, AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration,
        AEffectBuffScope, AEffectId, AItemListId,
    },
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
        adg_update_effect_fn: Some(update_effect),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, _a_warnings: &mut Vec<String>) {
    // Phenoms do not keep aggression though whole duration, but apply aggression only on initial
    // burst. Probably because of that, effect is not marked as offensive, even if aggression is
    // applied.
    a_effect.aggro = Some(AEffectAggroDuration::Instant);
}
