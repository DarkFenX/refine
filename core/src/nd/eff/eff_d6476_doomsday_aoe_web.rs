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

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_WEB;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_WEB;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            custom: vec![AEffectBuffCustom {
                buff_id: ac::buffs::STASIS_WEBIFICATION_BURST,
                source: AEffectBuffCustomSrc::Attr(ac::attrs::SPEED_FACTOR),
                scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
            }],
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
