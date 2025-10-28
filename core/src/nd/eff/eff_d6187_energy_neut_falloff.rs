use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{neut_opc::get_neut_opc, proj_mult::get_neut_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::ENERGY_NEUT_FALLOFF;
const A_EFFECT_ID: AEffectId = ac::effects::ENERGY_NEUT_FALLOFF;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            neut_opc_getter: Some(internal_get_neut_opc),
            ..
        },
        ..
    }
}

fn internal_get_neut_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    get_neut_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        projectee_key,
        get_neut_proj_mult,
    )
}
