use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectEcm, NEffectEcmChecker, NEffectEcmOutputGetter,
        NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutChecker, NEffectNeutKind, NEffectProjGetter,
        NEffectProjOpcSpec, NEffectResist,
    },
};

const EFFECT_AID: AEffectId = AEffectId::BOMB_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::Bomb),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Bomb,
            proj_mult_str: Some(NEffectProjGetter::BombApplication),
            proj_mult_chance: Some(NEffectProjGetter::BombRange),
            ..
        }),
        neut: Some(NEffectNeut {
            kind: NEffectNeutKind::Bomb,
            checker: Some(NEffectNeutChecker::Bomb),
            ospec: NEffectProjOpcSpec {
                base: NEffectGeneralOutputGetter::NeutBomb,
                proj_mult_str: Some(NEffectProjGetter::BombApplication),
                proj_mult_chance: Some(NEffectProjGetter::BombRange),
                resist: Some(NEffectResist::Standard),
                remote_limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
                ..
            },
        }),
        ecm: Some(NEffectEcm {
            checker: Some(NEffectEcmChecker::Bomb),
            ospec: NEffectProjOpcSpec {
                base: NEffectEcmOutputGetter::Bomb,
                proj_mult_chance: Some(NEffectProjGetter::BombRange),
                resist: Some(NEffectResist::Standard),
                ..
            },
        }),
        ..
    }
}
