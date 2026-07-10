use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjOpcSpec, NEffectProjecteeFilter},
};

const EFFECT_AID: AEffectId = AEffectId::LIGHTNING_WEAPON;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        projectee_filter: Some(NEffectProjecteeFilter::ItemListAttr(AAttrId::TGT_FILTER_TYPELIST_ID)),
        dmg_kind: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Delay1,
            ..
        }),
        ..
    }
}
