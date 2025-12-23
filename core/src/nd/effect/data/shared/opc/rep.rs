use crate::{
    ac,
    def::{AttrVal, OF},
    misc::{EffectSpec, Spool},
    nd::{NProjMultGetter, NSpoolResolver},
    rd::{RAttrKey, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Local reps
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_local_shield_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    _chargedness: Option<AttrVal>,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        effect,
        ctx.ac().shield_bonus,
        ctx.ac().shield_capacity,
        None,
        true,
    )
}

pub(in crate::nd::effect::data) fn get_local_armor_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    _chargedness: Option<AttrVal>,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        effect,
        ctx.ac().armor_dmg_amount,
        ctx.ac().armor_hp,
        None,
        false,
    )
}

pub(in crate::nd::effect::data) fn get_local_hull_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    _chargedness: Option<AttrVal>,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        effect,
        ctx.ac().struct_dmg_amount,
        ctx.ac().hp,
        None,
        false,
    )
}

pub(in crate::nd::effect::data) fn get_local_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    rep_attr_key: Option<RAttrKey>,
    limit_attr_key: Option<RAttrKey>,
    extra_mult: Option<AttrVal>,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_oattr_afb_odogma(ctx, item_key, rep_attr_key, OF(0.0))?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect)?,
    };
    if let Some(extra_mult) = extra_mult {
        amount *= extra_mult;
    }
    // Total resource pool limit
    if let Some(hp) = get_ship_attr(ctx, calc, item_key, limit_attr_key) {
        amount = amount.min(hp);
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

fn get_ship_attr(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, attr_key: Option<RAttrKey>) -> Option<AttrVal> {
    let fit_key = ctx.u_data.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.u_data.fits.get(fit_key).ship?;
    calc.get_item_oattr_oextra(ctx, ship_key, attr_key)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Remote reps
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_outgoing_shield_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        ctx.ac().shield_bonus,
        ctx.ac().shield_capacity,
        None,
        true,
    )
}

pub(in crate::nd::effect::data) fn get_outgoing_armor_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        ctx.ac().armor_dmg_amount,
        ctx.ac().armor_hp,
        None,
        false,
    )
}

pub(in crate::nd::effect::data) fn get_outgoing_hull_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        ctx.ac().struct_dmg_amount,
        ctx.ac().hp,
        None,
        false,
    )
}

pub(in crate::nd::effect::data) fn get_outgoing_cap_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        ctx.ac().power_transfer_amount,
        ctx.ac().capacitor_capacity,
        None,
        false,
    )
}

pub(in crate::nd::effect::data) fn get_outgoing_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
    amount_attr_key: Option<RAttrKey>,
    limit_attr_key: Option<RAttrKey>,
    extra_mult: Option<AttrVal>,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_oattr_afb_odogma(ctx, projector_key, amount_attr_key, OF(0.0))?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => eff_funcs::get_effect_duration_s(ctx, calc, projector_key, projector_effect)?,
    };
    if let Some(extra_mult) = extra_mult {
        amount *= extra_mult;
    }
    if let Some(spool_resolver) = spool_resolver
        && let Some(resolved_spool) = spool_resolver(ctx, calc, projector_key, projector_effect, spool)
    {
        amount *= resolved_spool.mult;
    }
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
        if let Some(hp) = calc.get_item_oattr_oextra(ctx, projectee_key, limit_attr_key) {
            amount = amount.min(hp);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////

pub(in crate::nd::effect::data) fn get_ancillary_armor_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    chargedness: Option<AttrVal>,
) -> Option<AttrVal> {
    if let Some(chargedness) = chargedness
        && let Some(charge_key) = ctx.u_data.items.get(item_key).get_charge_key()
        && ctx.u_data.items.get(charge_key).get_type_id() == ac::items::NANITE_REPAIR_PASTE
        && let Some(rep_mult) = calc.get_item_oattr_oextra(ctx, item_key, ctx.ac().charged_armor_dmg_mult)
    {
        return Some((rep_mult - OF(1.0)) * chargedness + OF(1.0));
    }
    None
}
