use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::EffectSpec,
    nd::{NEffect, effect::data::shared::proj_mult::get_aoe_burst_proj_mult},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_NEUT;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_NEUT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        neut_opc_getter: Some(internal_get_neut_opc),
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
    let attr_consts = ctx.ac();
    let mut amount = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.energy_neut_amount, OF(0.0))?;
    let delay = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.doomsday_warning_duration, OF(0.0))?;
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        amount *= get_aoe_burst_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        // Effect resistance reduction
        if let Some(neut_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
        {
            amount *= neut_mult;
        }
        // Total resource pool limit
        if let Some(cap) = calc.get_item_oattr_oextra(ctx, projectee_key, attr_consts.capacitor_capacity) {
            amount = amount.min(cap);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}
