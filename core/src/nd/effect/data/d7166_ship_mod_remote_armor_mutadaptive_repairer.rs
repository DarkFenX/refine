use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::Spool,
    nd::{
        NEffect,
        effect::{
            ResolvedSpool,
            data::shared::{opc::get_outgoing_armor_rep_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
        },
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        spool_resolver: Some(internal_get_resolved_spool),
        outgoing_armor_rep_opc_getter: Some(internal_get_outgoing_rep_opc),
        ..
    }
}

fn internal_get_resolved_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    spool: Option<Spool>,
) -> Option<ResolvedSpool> {
    ResolvedSpool::try_build(
        ctx,
        calc,
        item_key,
        effect,
        spool,
        ctx.ac().rep_mult_bonus_per_cycle,
        ctx.ac().rep_mult_bonus_max,
    )
}

fn internal_get_outgoing_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _chargedness: Option<AttrVal>,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    get_outgoing_armor_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        Some(internal_get_resolved_spool),
        projectee_key,
        get_simple_s2s_noapp_proj_mult,
    )
}
