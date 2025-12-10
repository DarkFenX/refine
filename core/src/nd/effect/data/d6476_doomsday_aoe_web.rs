use crate::{
    ac,
    ad::{AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_aoe_burst_mod_proj_attrs, get_aoe_burst_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_WEB;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_WEB;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff: Some(AEffectBuff {
            full: vec![AEffectBuffFull {
                buff_id: ac::buffs::STASIS_WEBIFICATION_BURST,
                strength: AEffectBuffStrength::Attr(ac::attrs::SPEED_FACTOR),
                duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
            }],
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_burst_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_aoe_burst_noapp_proj_mult),
        ..
    }
}
