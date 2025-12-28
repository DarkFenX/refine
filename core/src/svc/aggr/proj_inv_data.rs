use ordered_float::Float;

use crate::{
    def::{AttrVal, Count, OF},
    misc::{AttrSpec, EffectSpec},
    rd::{REffect, REffectProjOpcSpec, REffectResist},
    svc::{SvcCtx, calc::Calc, funcs, output::Output},
    ud::UItemKey,
    util::{FLOAT_TOLERANCE, ceil_unerr},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// General
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct ProjInvariantData<T>
where
    T: Copy,
{
    pub(super) output: Output<T>,
    pub(super) amount_limit: Option<AttrVal>,
    pub(super) mult_post: Option<AttrVal>,
}
impl<T> ProjInvariantData<T>
where
    T: Copy + std::ops::MulAssign<AttrVal>,
{
    pub(super) fn try_make(
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
            amount_limit = calc.get_item_oattr_oextra(ctx, projectee_key, ospec.ilimit_attr_key);
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
// Spool
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct SpoolInvariantData {
    pub(super) step: AttrVal,
    pub(super) max: AttrVal,
    pub(super) cycles_to_max: Count,
}
impl SpoolInvariantData {
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
