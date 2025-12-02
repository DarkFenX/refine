use itertools::chain;

use crate::{
    ac,
    ad::{AEffectBuffCustom, AEffectBuffCustomSrc, AEffectBuffInfo, AEffectBuffScope, AEffectId},
    def::OF,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{
            mods::make_dd_self_debuffs,
            opc::{get_aoe_dd_dmg_opc, get_aoe_dd_side_neut_opc},
            proj_mult::{get_aoe_dd_mod_proj_attrs, get_aoe_dd_noapp_proj_mult},
        },
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::DEBUFF_LANCE;
const A_EFFECT_ID: AEffectId = ac::effects::DEBUFF_LANCE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            custom: chain!(
                // Projected debuffs
                [
                    AEffectBuffCustom {
                        buff_id: ac::buffs::REMOTE_REPAIR_IMPEDANCE,
                        source: AEffectBuffCustomSrc::Hardcoded(OF(-50.0)),
                        scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
                    },
                    AEffectBuffCustom {
                        buff_id: ac::buffs::WARP_PENALTY,
                        source: AEffectBuffCustomSrc::Hardcoded(OF(100.0)),
                        scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
                    },
                    AEffectBuffCustom {
                        buff_id: ac::buffs::DISALLOW_DOCK_JUMP,
                        source: AEffectBuffCustomSrc::Hardcoded(OF(1.0)),
                        scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
                    },
                    AEffectBuffCustom {
                        buff_id: ac::buffs::DISALLOW_TETHER,
                        source: AEffectBuffCustomSrc::Hardcoded(OF(1.0)),
                        scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
                    },
                ],
                // Self-debuffs
                make_dd_self_debuffs()
            )
            .collect(),
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_dd_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_aoe_dd_noapp_proj_mult),
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(get_aoe_dd_dmg_opc),
            neut_opc_getter: Some(get_aoe_dd_side_neut_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
