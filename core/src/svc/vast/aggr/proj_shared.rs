use super::shared::process_mult;
use crate::{
    misc::{AttrSpec, EffectSpec},
    num::{Count, PValue, UnitInterval, Value},
    rd::{REffect, REffectProjOpcSpec, REffectResist},
    svc::{SvcCtx, calc::Calc, funcs, output::Output, vast::aggr::traits::LimitAmount},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// General data which stays the same through projected effect cycling
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct AggrProjInvData<T>
where
    T: Copy,
{
    pub(super) output: Output<T>,
    amount_limit: Option<Value>,
    mult_post: Option<PValue>,
}
impl<T> AggrProjInvData<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    pub(in crate::svc) fn try_make(
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_uid: UItemId,
        effect: &REffect,
        ospec: &REffectProjOpcSpec<T>,
        projectee_uid: Option<UItemId>,
    ) -> Option<Self> {
        let mut output = (ospec.base)(ctx, calc, projector_uid, effect)?;
        let mut amount_limit = None;
        let mut mult_post = None;
        if let Some(projectee_uid) = projectee_uid {
            let proj_data = ctx.eff_projs.get_or_make_proj_data(
                ctx.u_data,
                EffectSpec::new(projector_uid, effect.rid),
                projectee_uid,
            );
            let mut mult_pre = PValue::ONE;
            // Resists
            match ospec.resist {
                Some(REffectResist::Standard)
                    if let Some(resist_mult) =
                        funcs::get_effect_resist_mult(ctx, calc, projector_uid, effect, projectee_uid) =>
                {
                    mult_pre *= resist_mult;
                }
                Some(REffectResist::Attr(resist_attr_rid))
                    if let Some(resist_mult) = funcs::get_resist_mult_by_projectee_aspec(
                        ctx,
                        calc,
                        &AttrSpec::new(projectee_uid, resist_attr_rid),
                    ) =>
                {
                    mult_pre *= resist_mult;
                }
                _ => (),
            }
            // Strength-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_str {
                mult_pre *= proj_mult_getter(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
            }
            // Bake all pre-limit resists into output value
            if let Some(mult_pre) = process_mult(mult_pre) {
                output *= mult_pre;
            }
            // Amount limit
            amount_limit = calc.get_item_oattr_oextra(ctx, projectee_uid, ospec.limit_attr_rid);
            // Chance-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_chance {
                let mult = proj_mult_getter(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-related invariant data
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct AggrSpoolInvData {
    step: Value,
    pub(super) max: Value,
    pub(super) cycles_to_max: Count,
}
impl AggrSpoolInvData {
    pub(super) fn try_make<T>(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        ospec: &REffectProjOpcSpec<T>,
    ) -> Option<Self>
    where
        T: Copy,
    {
        if !ospec.spoolable {
            return None;
        }
        let spool_attr_rids = effect.spool_attr_rids?;
        let step = calc.get_item_attr_oextra(ctx, item_uid, spool_attr_rids.step_attr_rid)?;
        if step.abs() < PValue::FLOAT_TOLERANCE {
            return None;
        }
        let max = calc.get_item_attr_oextra(ctx, item_uid, spool_attr_rids.max_attr_rid)?;
        if max.abs() < PValue::FLOAT_TOLERANCE {
            return None;
        }
        let cycles = max / step;
        if cycles.is_sign_negative() {
            return None;
        }
        Some(Self {
            step,
            max,
            cycles_to_max: Count::from_value_ceiled(cycles),
        })
    }
    pub(super) fn calc_cycle_spool(&self, uninterrupted_cycles: Count) -> Value {
        (self.step * uninterrupted_cycles.into_value()).min(self.max)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) fn get_proj_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: &AggrProjInvData<T>,
    chargedness: Option<UnitInterval>,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitAmount,
{
    let mut output = inv_proj.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargedness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_uid, chargedness)
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
    charge_mult: Option<PValue>,
    spool_extra_mult: Value,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitAmount,
{
    let mut output = inv_proj.output;
    // Chargedness
    if let Some(charge_mult) = charge_mult {
        output *= charge_mult;
    }
    // Spool
    output *= PValue::from_value_clamped(Value::ONE + spool_extra_mult);
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
