use super::shared::make_dd_self_debuffs;
use crate::{
    ad::{AEffectBuff, AEffectId, AItemListId},
    ed::EEffectId,
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjOpcSpec, NEffectProjecteeFilter},
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
        dmg_kind: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Delay1,
            ..
        }),
        ..
    }
}
