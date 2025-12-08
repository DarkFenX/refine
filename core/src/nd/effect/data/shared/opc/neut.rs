use crate::{
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    nd::{NProjMultGetter, effect::data::shared::proj_mult::get_aoe_dd_side_neut_proj_mult},
    rd::{RAttrKey, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(in crate::nd::effect::data) fn get_generic_neut_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
    amount_attr_key: Option<RAttrKey>,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_oattr_afb_oextra(ctx, projector_key, amount_attr_key, OF(0.0))?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => eff_funcs::get_effect_duration_s(ctx, calc, projector_key, projector_effect)?,
    };
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
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
        if let Some(cap) = calc.get_item_oattr_oextra(ctx, projectee_key, ctx.ac().capacitor_capacity) {
            amount = amount.min(cap);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

pub(in crate::nd::effect::data) fn get_aoe_dd_side_neut_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    let attr_consts = ctx.ac();
    let mut amount =
        calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.doomsday_energy_neut_amount, OF(0.0))?;
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        amount *= get_aoe_dd_side_neut_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        // Effect resistance reduction
        if let Some(resist_attr_key) = attr_consts.doomsday_energy_neut_resist_id
            && let Some(resist_mult) =
                eff_funcs::get_resist_mult_by_projectee_aspec(ctx, calc, &AttrSpec::new(projectee_key, resist_attr_key))
        {
            amount *= resist_mult;
        }
        // Total resource pool limit
        if let Some(cap) = calc.get_item_oattr_oextra(ctx, projectee_key, attr_consts.capacitor_capacity) {
            amount = amount.min(cap);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay: OF(0.0) }))
}
