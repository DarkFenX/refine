// Abyssal white cloud

use crate::{
    ac,
    ad::{AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::AOE_BEACON_FILAMENT_CLOUD;
const EFFECT_AID: AEffectId = AEffectId::AOE_BEACON_FILAMENT_CLOUD;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::None,
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
            }),
            ..
        }),
        ..
    }
}
