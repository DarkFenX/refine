use crate::{
    ad::{AEffectBuff, AEffectId, AItemListId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKindGetter, NEffectProjecteeFilter,
        effect::data::shared::{base_opc::get_direct_dd_dmg_opc_spec, mods::make_dd_self_debuffs},
    },
};

const EFFECT_EID: EEffectId = EEffectId::SUPER_WEAPON_MINMATAR;
const EFFECT_AID: AEffectId = AEffectId::SUPER_WEAPON_MINMATAR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        projectee_filter: Some(NEffectProjecteeFilter::ItemList(AItemListId::CAPITALS_FREIGHTERS)),
        dmg_kind_getter: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg_opc_spec: Some(get_direct_dd_dmg_opc_spec()),
        ..
    }
}
