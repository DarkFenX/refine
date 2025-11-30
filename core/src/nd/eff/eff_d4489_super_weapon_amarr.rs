use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{mods::add_dd_mods, opc::get_direct_dd_dmg_opc},
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::SUPER_WEAPON_AMARR;
const A_EFFECT_ID: AEffectId = ac::effects::SUPER_WEAPON_AMARR;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| add_dd_mods(A_EFFECT_ID, a_effect, false)),
        hc: NEffectHc {
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(get_direct_dd_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
