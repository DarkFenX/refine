use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectHc, NEffectProjecteeFilter,
        eff::shared::{mods::make_dd_self_debuffs, opc::get_direct_dd_dmg_opc},
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::SUPER_WEAPON_CALDARI;
const A_EFFECT_ID: AEffectId = ac::effects::SUPER_WEAPON_CALDARI;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            custom: make_dd_self_debuffs().collect(),
            ..
        }),
        hc: NEffectHc {
            projectee_filter: Some(NEffectProjecteeFilter::ItemList(ac::itemlists::CAPITALS_FREIGHTERS)),
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
