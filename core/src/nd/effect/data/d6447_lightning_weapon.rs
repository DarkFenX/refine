use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjOpcSpec, NEffectProjecteeFilter},
};

const EFFECT_EID: EEffectId = EEffectId::LIGHTNING_WEAPON;
const EFFECT_AID: AEffectId = AEffectId::LIGHTNING_WEAPON;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        projectee_filter: Some(NEffectProjecteeFilter::ItemListAttr(AAttrId::TGT_FILTER_TYPELIST_ID)),
        dmg_kind_getter: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Delay1,
            ..
        }),
        ..
    }
}
