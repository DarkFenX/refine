use crate::{
    ac,
    ad::{AEffectBuffDuration, AEffectBuffFull, AEffectBuffInfo, AEffectBuffScope, AEffectBuffStrength, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        effect::data::shared::proj_mult::{get_aoe_burst_mod_proj_attrs, get_aoe_burst_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_DAMP;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_DAMP;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            full: vec![
                AEffectBuffFull {
                    buff_id: ac::buffs::DAMP_BURST_TARGETING_RANGE_PENALTY,
                    strength: AEffectBuffStrength::Attr(ac::attrs::MAX_TARGET_RANGE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::DAMP_BURST_SCAN_RESOLUTION_PENALTY,
                    strength: AEffectBuffStrength::Attr(ac::attrs::SCAN_RESOLUTION_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
            ],
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_burst_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_aoe_burst_noapp_proj_mult),
            ..
        },
        ..
    }
}
