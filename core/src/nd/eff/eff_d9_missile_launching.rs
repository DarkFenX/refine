use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{dmg_opc::get_dmg_opc_missile, proj_mult::get_proj_mult_missile},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::MISSILE_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Missile),
            normal_dmg_opc_getter: Some(internal_get_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_r_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    get_dmg_opc_missile(
        ctx,
        calc,
        projector_key,
        projector_r_effect,
        projectee_key,
        get_proj_mult_missile,
    )
}
