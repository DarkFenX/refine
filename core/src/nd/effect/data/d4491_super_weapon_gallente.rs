use crate::{
    ac,
    ad::{AEffectBuff, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjecteeFilter,
        effect::data::shared::{base_opc::get_direct_dd_dmg_opc_spec, mods::make_dd_self_debuffs},
    },
    ud::UItem,
};

const EFFECT_EID: EEffectId = ec::effects::SUPER_WEAPON_GALLENTE;
const EFFECT_AID: AEffectId = ac::effects::SUPER_WEAPON_GALLENTE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        projectee_filter: Some(NEffectProjecteeFilter::ItemList(ac::itemlists::CAPITALS_FREIGHTERS)),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(get_direct_dd_dmg_opc_spec()),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
