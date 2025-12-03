use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        effect::shared::{opc::get_generic_neut_opc, proj_mult::get_neut_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::ENERGY_NOSF_FALLOFF;
const A_EFFECT_ID: AEffectId = ac::effects::ENERGY_NOSF_FALLOFF;

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
    // Not a blood raider ship - not considered as a neut
    if calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::NOS_OVERRIDE)? == OF(0.0) {
        return None;
    }
    get_generic_neut_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        projectee_key,
        get_neut_proj_mult,
        &ac::attrs::POWER_TRANSFER_AMOUNT,
        false,
    )
}
