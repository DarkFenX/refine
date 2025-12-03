use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::Spool,
    nd::{
        NEffect, NEffectHc,
        effect::data::shared::{opc::get_outgoing_hull_rep_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::NPC_ENTITY_REMOTE_HULL_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::NPC_ENTITY_REMOTE_HULL_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            outgoing_hull_rep_opc_getter: Some(internal_get_outgoing_rep_opc),
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
    get_outgoing_hull_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        None,
        projectee_key,
        get_simple_s2s_noapp_proj_mult,
    )
}
