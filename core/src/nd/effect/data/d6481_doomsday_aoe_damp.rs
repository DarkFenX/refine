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

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_DAMP;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_DAMP;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![
                AEffectBuffFull {
                    buff_id: ABuffId::DAMP_BURST_TARGETING_RANGE_PENALTY,
                    strength: AEffectBuffStrength::Attr(AAttrId::MAX_TARGET_RANGE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::DAMP_BURST_SCAN_RESOLUTION_PENALTY,
                    strength: AEffectBuffStrength::Attr(AAttrId::SCAN_RESOLUTION_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
            ],
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_burst_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_aoe_burst_noapp_proj_mult),
        ..
    }
}
