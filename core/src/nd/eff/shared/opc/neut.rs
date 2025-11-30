use crate::{
    ac,
    ad::AAttrId,
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    nd::{NProjMultGetter, eff::shared::proj_mult::get_dd_neut_proj_mult},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(in crate::nd::eff) fn get_generic_neut_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
    amount_attr_id: &AAttrId,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_key, amount_attr_id)?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => eff_funcs::get_effect_duration_s(ctx, calc, projector_key, projector_effect)?,
    };
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        amount *= proj_mult_getter(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        // Effect resistance reduction
        if let Some(rr_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
        {
            amount *= rr_mult;
        }
        // Total resource pool limit
        if let Some(cap) = calc.get_item_attr_val_extra_opt(ctx, projectee_key, &ac::attrs::CAPACITOR_CAPACITY) {
            amount = amount.min(cap);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

pub(in crate::nd::eff) fn get_aoe_dd_neut_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_ENERGY_NEUT_AMOUNT)?;
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        amount *= get_dd_neut_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        // Effect resistance reduction
        let projectee_aspec = AttrSpec::new(projectee_key, ac::attrs::DOOMSDAY_ENERGY_NEUT_RESIST_ID);
        if let Some(resist_mult) = eff_funcs::get_resist_mult_val_by_projectee_aspec(ctx, calc, &projectee_aspec) {
            amount *= resist_mult;
        }
        // Total resource pool limit
        if let Some(cap) = calc.get_item_attr_val_extra_opt(ctx, projectee_key, &ac::attrs::CAPACITOR_CAPACITY) {
            amount = amount.min(cap);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay: OF(0.0) }))
}
