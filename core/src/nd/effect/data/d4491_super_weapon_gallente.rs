use crate::{
    ac,
    ad::{AEffectBuff, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjecteeFilter,
        effect::data::shared::{base_opc::get_direct_dd_dmg_opc, mods::make_dd_self_debuffs},
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::SUPER_WEAPON_GALLENTE;
const A_EFFECT_ID: AEffectId = ac::effects::SUPER_WEAPON_GALLENTE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        projectee_filter: Some(NEffectProjecteeFilter::ItemList(ac::itemlists::CAPITALS_FREIGHTERS)),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_getter: Some(get_direct_dd_dmg_opc),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
