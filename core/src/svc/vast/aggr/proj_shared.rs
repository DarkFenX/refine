use crate::{
    misc::{AttrSpec, EffectSpec},
    num::{Count, PValue, UnitInterval, Value},
    rd::{REffect, REffectProjOpcSpec, REffectResist},
    svc::{SvcCtx, calc::Calc, funcs, output::Output, vast::aggr::traits::LimitInstance},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// General data which stays the same through projected effect cycling
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct AggrProjInvData<T>
where
    T: Copy,
{
    pub(super) base_output: Output<T>,
    is_nulled: bool,
    pub(super) str_mult: PValue,
    instance_limit: Option<Value>,
    pub(super) chance_mult: Option<PValue>,
}
impl<T> AggrProjInvData<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    pub(in crate::svc::vast) fn try_make(
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_uid: UItemId,
        effect: &REffect,
        ospec: &REffectProjOpcSpec<T>,
        projectee_uid: Option<UItemId>,
    ) -> Option<Self> {
        let base_output = (ospec.base)(ctx, calc, projector_uid, effect)?;
        let mut str_mult = PValue::ONE;
        let mut chance_mult = PValue::ONE;
        let mut instance_limit = None;
        if let Some(projectee_uid) = projectee_uid {
            let proj_data = ctx.eff_projs.get_or_make_proj_data(
                ctx.u_data,
                EffectSpec::new(projector_uid, effect.rid),
                projectee_uid,
            );
            // Amount limit
            instance_limit = calc.get_item_oattr_oextra(ctx, projectee_uid, ospec.limit_attr_rid);
            // Strength-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_str {
                str_mult *= proj_mult_getter(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
            }
            if str_mult == PValue::ZERO {
                return Some(Self::make_nulled(base_output, instance_limit));
            }
            // Chance-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_chance {
                chance_mult *= proj_mult_getter(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
            }
            if chance_mult == PValue::ZERO {
                return Some(Self::make_nulled(base_output, instance_limit));
            }
            // Resists
            if let Some(resist) = ospec.resist {
                let resist_mult = match resist {
                    REffectResist::Standard => {
                        funcs::get_effect_resist_mult(ctx, calc, projector_uid, effect, projectee_uid)
                    }
                    REffectResist::Attr(resist_attr_rid) => funcs::get_resist_mult_by_projectee_aspec(
                        ctx,
                        calc,
                        &AttrSpec::new(projectee_uid, resist_attr_rid),
                    ),
                };
                match resist_mult {
                    Some(PValue::ZERO) => return Some(Self::make_nulled(base_output, instance_limit)),
                    Some(resist_mult) => str_mult *= resist_mult,
                    None => (),
                }
            }
        }
        Some(Self {
            base_output,
            is_nulled: false,
            str_mult,
            instance_limit,
            chance_mult: process_mult(chance_mult),
        })
    }
    fn make_nulled(mut base_output: Output<T>, instance_limit: Option<Value>) -> Self {
        base_output *= PValue::ZERO;
        Self {
            base_output,
            is_nulled: true,
            str_mult: PValue::ZERO,
            instance_limit,
            chance_mult: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-related invariant data
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
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
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct ProjConverterRegular<'sc1, 'sc2, 'calc, 'ospec, 'ip, T>
where
    T: Copy,
{
    pub(super) ctx: SvcCtx<'sc1, 'sc2>,
    pub(super) calc: &'calc mut Calc,
    pub(super) projector_uid: UItemId,
    pub(super) ospec: &'ospec REffectProjOpcSpec<T>,
    pub(super) inv_proj: &'ip AggrProjInvData<T>,
}
impl<'sc1, 'sc2, 'calc, 'ospec, 'ip, T> ProjConverterRegular<'sc1, 'sc2, 'calc, 'ospec, 'ip, T>
where
    T: Copy,
{
    pub(super) fn new(
        ctx: SvcCtx<'sc1, 'sc2>,
        calc: &'calc mut Calc,
        projector_uid: UItemId,
        ospec: &'ospec REffectProjOpcSpec<T>,
        inv_proj: &'ip AggrProjInvData<T>,
    ) -> Self {
        Self {
            ctx,
            calc,
            projector_uid,
            ospec,
            inv_proj,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) fn get_proj_regular_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: &AggrProjInvData<T>,
    chargedness: Option<UnitInterval>,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
{
    let mut output = inv_proj.base_output;
    let mut str_mult = inv_proj.str_mult;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargedness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_uid, chargedness)
    {
        str_mult *= charge_mult;
    }
    if str_mult != PValue::ONE {
        output *= str_mult;
    }
    // Limit
    if let Some(limit) = inv_proj.instance_limit {
        output.limit_instance(limit);
    }
    output
}

pub(super) fn get_proj_spool_part_str_mult<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: &AggrProjInvData<T>,
    chargedness: Option<UnitInterval>,
) -> PValue
where
    T: Copy,
{
    let mut str_mult = inv_proj.str_mult;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargedness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_uid, chargedness)
    {
        str_mult *= charge_mult;
    }
    str_mult
}

pub(super) fn get_proj_spool_cycle_output<T>(
    inv_proj: &AggrProjInvData<T>,
    mut str_mult: PValue,
    spool_extra_mult: Value,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
{
    let mut output = inv_proj.base_output;
    // Spool
    str_mult *= PValue::from_value_clamped(Value::ONE + spool_extra_mult);
    if str_mult != PValue::ONE {
        output *= str_mult;
    }
    // Limit
    if let Some(instance_limit) = inv_proj.instance_limit {
        output.limit_instance(instance_limit);
    }
    output
}

fn process_mult(mult: PValue) -> Option<PValue> {
    match mult {
        PValue::ONE => None,
        v => Some(v),
    }
}
