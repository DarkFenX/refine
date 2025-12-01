use crate::{
    ac,
    ad::{AEffectBuffCustom, AEffectBuffCustomSrc, AEffectBuffInfo, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_aoe_burst_mod_proj_attrs, get_noapp_aoe_burst_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_DAMP;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_DAMP;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            custom: vec![
                AEffectBuffCustom {
                    buff_id: ac::buffs::DAMP_BURST_TARGETING_RANGE_PENALTY,
                    source: AEffectBuffCustomSrc::Attr(ac::attrs::MAX_TARGET_RANGE_BONUS),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
                },
                AEffectBuffCustom {
                    buff_id: ac::buffs::DAMP_BURST_SCAN_RESOLUTION_PENALTY,
                    source: AEffectBuffCustomSrc::Attr(ac::attrs::SCAN_RESOLUTION_BONUS),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
                },
            ],
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_burst_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_noapp_aoe_burst_proj_mult),
            ..
        },
        ..
    }
}
