// Abyssal tracking towers

use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectId},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::AOE_BEACON_PULSE_01;
const A_EFFECT_ID: AEffectId = ac::effects::AOE_BEACON_PULSE_01;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            source: AEffectBuffSrc::DefaultAttrs,
            scope: AEffectBuffScope {
                item_list_id: ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS,
                ..
            },
        }),
        ..
    }
}
