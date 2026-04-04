use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectEcm, NEffectEcmChecker, NEffectEcmOutputGetter,
        NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutChecker, NEffectNeutKind, NEffectProjMultGetter,
        NEffectProjOpcSpec, NEffectResist,
    },
};

const EFFECT_EID: EEffectId = EEffectId::BOMB_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::BOMB_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::Bomb),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjMultGetter::BombApplication),
            proj_mult_chance: Some(NEffectProjMultGetter::BombRange),
            ..
        }),
        neut: Some(NEffectNeut {
            kind: NEffectNeutKind::Bomb,
            checker: Some(NEffectNeutChecker::Bomb),
            ospec: NEffectProjOpcSpec {
                base: NEffectGeneralOutputGetter::NeutBomb,
                proj_mult_str: Some(NEffectProjMultGetter::BombApplication),
                proj_mult_chance: Some(NEffectProjMultGetter::BombRange),
                resist: Some(NEffectResist::Standard),
                remote_limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
                ..
            },
        }),
        ecm: Some(NEffectEcm {
            checker: Some(NEffectEcmChecker::Bomb),
            ospec: NEffectProjOpcSpec {
                base: NEffectEcmOutputGetter::Bomb,
                proj_mult_chance: Some(NEffectProjMultGetter::BombRange),
                resist: Some(NEffectResist::Standard),
                ..
            },
        }),
        ..
    }
}
