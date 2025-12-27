use super::traits::Aggregable;
use crate::{
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    rd::{REffect, REffectProjOpcSpec, REffectResist},
    svc::{SvcCtx, calc::Calc, funcs, output::Output},
    ud::UItemKey,
};

pub(super) struct ProjInvariantData<T>
where
    T: Copy,
{
    pub(super) output: Output<T>,
    pub(super) amount_limit: Option<AttrVal>,
    pub(super) mult_post: Option<AttrVal>,
}

pub(super) fn try_make_proj_inv_data<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<ProjInvariantData<T>>
where
    T: Copy + Aggregable,
{
    let mut output = (ospec.base)(ctx, calc, projector_key, effect)?;
    let mut amount_limit = None;
    let mut mult_post = None;
    if let Some(projectee_key) = projectee_key {
        let proj_data =
            ctx.eff_projs
                .get_or_make_proj_data(ctx.u_data, EffectSpec::new(projector_key, effect.key), projectee_key);
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
    Some(ProjInvariantData {
        output,
        amount_limit,
        mult_post,
    })
}

fn process_mult(mult: AttrVal) -> Option<AttrVal> {
    match mult {
        OF(1.0) => None,
        v => Some(v),
    }
}
