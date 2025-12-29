use ordered_float::Float;

use crate::{
    def::{AttrVal, Count, OF},
    misc::{AttrSpec, EffectSpec},
    rd::{REffect, REffectProjOpcSpec, REffectResist},
    svc::{SvcCtx, aggr::traits::LimitAmount, calc::Calc, funcs, output::Output},
    ud::UItemKey,
    util::{FLOAT_TOLERANCE, ceil_unerr},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// General data which stays the same through projected effect cycling
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct AggrProjInvData<T>
where
    T: Copy,
{
    output: Output<T>,
    amount_limit: Option<AttrVal>,
    mult_post: Option<AttrVal>,
}
impl<T> AggrProjInvData<T>
where
    T: Copy + std::ops::MulAssign<AttrVal>,
{
    pub(in crate::svc) fn try_make(
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_key: UItemKey,
        effect: &REffect,
        ospec: &REffectProjOpcSpec<T>,
        projectee_key: Option<UItemKey>,
    ) -> Option<Self> {
        let mut output = (ospec.base)(ctx, calc, projector_key, effect)?;
        let mut amount_limit = None;
        let mut mult_post = None;
        if let Some(projectee_key) = projectee_key {
            let proj_data = ctx.eff_projs.get_or_make_proj_data(
                ctx.u_data,
                EffectSpec::new(projector_key, effect.key),
                projectee_key,
            );
            let mut mult_pre = OF(1.0);
            // Resists
            match ospec.resist {
                Some(REffectResist::Standard)
                    if let Some(resist_mult) =
                        funcs::get_effect_resist_mult(ctx, calc, projector_key, effect, projectee_key) =>
                {
                    mult_pre *= resist_mult;
                }
                Some(REffectResist::Attr(resist_attr_key))
                    if let Some(resist_mult) = funcs::get_resist_mult_by_projectee_aspec(
                        ctx,
                        calc,
                        &AttrSpec::new(projectee_key, resist_attr_key),
                    ) =>
                {
                    mult_pre *= resist_mult;
                }
                _ => (),
            }
            // Strength-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_str {
                mult_pre *= proj_mult_getter(ctx, calc, projector_key, effect, projectee_key, proj_data);
            }
            // Bake all pre-limit resists into output value
            if let Some(mult_pre) = process_mult(mult_pre) {
                output *= mult_pre;
            }
            // Amount limit
            amount_limit = calc.get_item_oattr_oextra(ctx, projectee_key, ospec.limit_attr_key);
            // Chance-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_chance {
                let mult = proj_mult_getter(ctx, calc, projector_key, effect, projectee_key, proj_data);
                mult_post = process_mult(mult);
            }
        }
        Some(Self {
            output,
            amount_limit,
            mult_post,
        })
    }
}

fn process_mult(mult: AttrVal) -> Option<AttrVal> {
    match mult {
        OF(1.0) => None,
        v => Some(v),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-related invariant data
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct AggrSpoolInvData {
    pub(super) step: AttrVal,
    pub(super) max: AttrVal,
    pub(super) cycles_to_max: Count,
}
impl AggrSpoolInvData {
    pub(super) fn try_make<T>(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        effect: &REffect,
        ospec: &REffectProjOpcSpec<T>,
    ) -> Option<Self>
    where
        T: Copy,
    {
        if !ospec.spoolable {
            return None;
        }
        let spool_attr_keys = effect.spool_attr_keys?;
        let step = calc.get_item_attr_oextra(ctx, item_key, spool_attr_keys.step)?;
        if step.abs() < FLOAT_TOLERANCE {
            return None;
        }
        let max = calc.get_item_attr_oextra(ctx, item_key, spool_attr_keys.max)?;
        if max.abs() < FLOAT_TOLERANCE {
            return None;
        }
        let cycles = max / step;
        if cycles.is_sign_negative() {
            return None;
        }
        Some(Self {
            step,
            max,
            cycles_to_max: ceil_unerr(cycles).into_inner() as Count,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) fn get_proj_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: &AggrProjInvData<T>,
    chargeness: Option<AttrVal>,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    let mut output = inv_proj.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargeness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
    {
        output *= charge_mult;
    }
    // Limit
    if let Some(limit) = inv_proj.amount_limit {
        output.limit_amount(limit);
    }
    // Chance-based multipliers
    if let Some(mult_post) = inv_proj.mult_post {
        output *= mult_post;
    }
    output
}

pub(super) fn get_proj_output_spool<T>(
    inv_proj: &AggrProjInvData<T>,
    charge_mult: Option<AttrVal>,
    spool_extra_mult: AttrVal,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    let mut output = inv_proj.output;
    // Chargedness
    if let Some(charge_mult) = charge_mult {
        output *= charge_mult;
    }
    // Spool
    output *= OF(1.0) + spool_extra_mult;
    // Limit
    if let Some(limit) = inv_proj.amount_limit {
        output.limit_amount(limit);
    }
    // Chance-based multipliers
    if let Some(mult_post) = inv_proj.mult_post {
        output *= mult_post;
    }
    output
}
