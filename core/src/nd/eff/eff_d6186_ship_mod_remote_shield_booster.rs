use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::Spool,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{opc::get_outgoing_shield_rep_opc, proj_mult::get_noapp_full_proj_mult},
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
            outgoing_shield_rep_opc_getter: Some(internal_get_outgoing_rep_opc),
            ..
        },
        ..
    }
}

fn internal_get_outgoing_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    get_outgoing_shield_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        None,
        projectee_key,
        get_noapp_full_proj_mult,
    )
}
