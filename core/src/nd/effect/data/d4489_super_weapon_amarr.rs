use crate::{
    ad::{AEffectBuff, AEffectId, AItemListId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjOpcSpec, NEffectProjecteeFilter,
        effect::data::shared::mods::make_dd_self_debuffs,
    },
};

const EFFECT_EID: EEffectId = EEffectId::SUPER_WEAPON_AMARR;
const EFFECT_AID: AEffectId = AEffectId::SUPER_WEAPON_AMARR;

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
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Delay1,
            ..
        }),
        ..
    }
}
