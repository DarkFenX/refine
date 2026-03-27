use itertools::chain;

use super::shared::{get_aoe_dd_side_neut_ospec, make_dd_self_debuffs};
use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItemListId, AValue,
    },
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectModProjAttrsGetter, NEffectProjMultGetter,
        NEffectProjOpcSpec,
    },
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
        modifier_proj_attrs_getter: Some(NEffectModProjAttrsGetter::AoeDd),
        modifier_proj_mult_getter: Some(NEffectProjMultGetter::AoeDdRange),
        dmg_kind_getter: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::DotDelay,
            proj_mult_str: Some(NEffectProjMultGetter::AoeDd),
            ..
        }),
        neut_opc_spec: Some(get_aoe_dd_side_neut_ospec()),
        ..
    }
}
