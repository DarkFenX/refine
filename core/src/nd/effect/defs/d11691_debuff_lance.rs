use itertools::chain;

use super::shared::{get_aoe_dd_warmup_neut, make_dd_self_debuffs};
use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength, AItemListId, AValue,
    },
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DEBUFF_LANCE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: chain(
                // Projected debuffs
                [
                    AEffectBuffFull {
                        buff_id: ABuffId::REMOTE_REPAIR_IMPEDANCE,
                        strength: AEffectModStrength::Hardcoded(AValue::from_f64(-50.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                    },
                    AEffectBuffFull {
                        buff_id: ABuffId::WARP_PENALTY,
                        strength: AEffectModStrength::Hardcoded(AValue::from_f64(100.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                    },
                    AEffectBuffFull {
                        buff_id: ABuffId::DISALLOW_DOCK_JUMP,
                        strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                    },
                    AEffectBuffFull {
                        buff_id: ABuffId::DISALLOW_TETHER,
                        strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
                        duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_APPLIED_DBUFF_DURATION),
                        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                    },
                ],
                // Self-debuffs
                make_dd_self_debuffs(),
            )
            .collect(),
            ..
        }),
        modifier_proj: Some(NEffectProjGetter::AoeDdRange),
        dmg_kind: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::DotDelay,
            proj_mult_str: Some(NEffectProjGetter::AoeDd),
            ..
        }),
        neut: Some(get_aoe_dd_warmup_neut()),
        ..
    }
}
