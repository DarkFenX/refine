use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::Spool,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{proj_mult::get_proj_mult_normal_restricted_s2s, rep_opc::get_remote_shield_rep_opc},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_SHIELD_BOOSTER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_SHIELD_BOOSTER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            remote_shield_rep_opc_getter: Some(internal_get_remote_rep_opc),
            ..
        },
        ..
    }
}

fn internal_get_remote_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_r_effect: &REffect,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    get_remote_shield_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_r_effect,
        spool,
        None,
        projectee_key,
        get_proj_mult_normal_restricted_s2s,
    )
}
