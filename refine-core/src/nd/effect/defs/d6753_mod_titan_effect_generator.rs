use crate::{
    ad::{
        AAttrId, AEffect, AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId,
        AEffectWeaponsTimerApplication, AItemListId,
    },
    nd::{NEffect, NEffectProjModSpec, NEffectProjMultGetter},
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
            proj_mult: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, _a_warnings: &mut Vec<String>) {
    // Phenoms do not keep refreshing weapons timer though whole cycle duration, but apply it only
    // on initial burst. Probably because of that, effect is not marked as offensive, even if
    // weapons timer is applied.
    a_effect.weapons_timer = Some(AEffectWeaponsTimerApplication::Instant);
}
