use itertools::chain;

use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItemListId, AValue,
    },
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind,
        effect::data::shared::{
            base_opc::{get_aoe_dd_dmg_opc_spec, get_aoe_dd_side_neut_opc_spec},
            mods::make_dd_self_debuffs,
            proj_mult::{get_aoe_dd_mod_proj_attrs, get_aoe_dd_noapp_proj_mult},
        },
    },
    ud::UItem,
};

const EFFECT_EID: EEffectId = EEffectId::DEBUFF_LANCE;
const EFFECT_AID: AEffectId = AEffectId::DEBUFF_LANCE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: chain(
                // Projected debuffs
                [
                    AEffectBuffFull {
                        buff_id: ABuffId::REMOTE_REPAIR_IMPEDANCE,
                        strength: AEffectBuffStrength::Hardcoded(AValue::from_f64(-50.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                    },
                    AEffectBuffFull {
                        buff_id: ABuffId::WARP_PENALTY,
                        strength: AEffectBuffStrength::Hardcoded(AValue::from_f64(100.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                    },
                    AEffectBuffFull {
                        buff_id: ABuffId::DISALLOW_DOCK_JUMP,
                        strength: AEffectBuffStrength::Hardcoded(AValue::from_f64(1.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                    },
                    AEffectBuffFull {
                        buff_id: ABuffId::DISALLOW_TETHER,
                        strength: AEffectBuffStrength::Hardcoded(AValue::from_f64(1.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                    },
                ],
                // Self-debuffs
                make_dd_self_debuffs(),
            )
            .collect(),
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_dd_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_aoe_dd_noapp_proj_mult),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(get_aoe_dd_dmg_opc_spec()),
        neut_opc_spec: Some(get_aoe_dd_side_neut_opc_spec()),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
