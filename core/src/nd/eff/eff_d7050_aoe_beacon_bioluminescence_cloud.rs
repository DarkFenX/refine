// Abyssal blue cloud

use crate::{
    ac,
    ad::{AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffInfo, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::AOE_BEACON_BIOLUMINESCENCE_CLOUD;
const A_EFFECT_ID: AEffectId = ac::effects::AOE_BEACON_BIOLUMINESCENCE_CLOUD;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::None,
                scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
            }),
            ..
        }),
        ..
    }
}
