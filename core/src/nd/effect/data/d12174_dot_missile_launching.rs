use crate::{
    ad::AEffectId,
    def::SERVER_TICK_S,
    ed::EEffectId,
    misc::Breacher,
    nd::{NBaseBreacherDmgGetter, NEffect, NEffectDmgKindGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
    num::{Count, PValue, UnitInterval, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

const EFFECT_EID: EEffectId = EEffectId::DOT_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::DOT_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind_getter: Some(NEffectDmgKindGetter::Breacher),
        breacher_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: NBaseBreacherDmgGetter::Regular,
            proj_mult_chance: Some(NEffectProjMultGetter::MissileRange),
            ..
        }),
        ..
    }
}
