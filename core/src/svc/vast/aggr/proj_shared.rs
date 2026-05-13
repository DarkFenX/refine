use super::{
    shared::{AggrPartDataTail, get_item_ship_limit},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    misc::{AttrSpec, EffectSpec},
    nd::NEffectOutputGetter,
    num::{Count, PValue, UnitInterval, Value},
    rd::{REffect, REffectProjOpcSpec, REffectResist},
    svc::{SvcCtx, calc::Calc, cycle::CycleDataFull, funcs, output::Output},
    ud::UItemId,
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// General data which stays the same through projected effect cycling
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(super) struct AggrProjInvData<T>
where
    T: Copy,
{
    // TODO: consider if fields can be made private (and check if base output users use it properly)
    pub(super) base_output: Output<T>,
    pub(super) str_mult: PValue,
    instance_limit: Option<PValue>,
    pub(super) chance_mult: Option<PValue>,
}
impl<T> AggrProjInvData<T>
where
    T: Copy + std::ops::MulAssign<PValue> + HasImpact,
{
    pub(super) fn try_make<BG, BX>(
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_uid: UItemId,
        effect: &REffect,
        ospec: &REffectProjOpcSpec<BG>,
        base_xargs: BX,
        projectee_uid: Option<UItemId>,
    ) -> Option<Self>
    where
        BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    {
        let base_output = ospec.base.get(ctx, calc, projector_uid, effect, base_xargs)?;
        if !base_output.has_impact() || base_output.get_instance_count() == Count::ZERO {
            return None;
        }
        let mut str_mult = PValue::ONE;
        let mut chance_mult = PValue::ONE;
        let mut instance_limit = get_item_ship_limit(ctx, calc, projector_uid, ospec.local_limit_attr_id);
        if let Some(projectee_uid) = projectee_uid {
            let proj_data = ctx.eff_projs.get_or_make_proj_data(
                ctx.u_data,
                EffectSpec::new(projector_uid, effect.rid),
                projectee_uid,
            );
            // Remote limit
            if let Some(remote_limit) = calc.get_item_oattr_oextra(ctx, projectee_uid, ospec.remote_limit_attr_id) {
                let remote_limit = PValue::from_value_clamped(remote_limit);
                match instance_limit {
                    Some(local_limit) => instance_limit = Some(local_limit.min(remote_limit)),
                    None => instance_limit = Some(remote_limit),
                }
            }
            // Strength-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_str {
                let proj_mult = proj_mult_getter.get_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
                if proj_mult == PValue::ZERO {
                    return None;
                }
                str_mult *= proj_mult;
            }
            // Chance-modifying projection
            if let Some(proj_mult_getter) = ospec.proj_mult_chance {
                let proj_mult = proj_mult_getter.get_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
                if proj_mult == PValue::ZERO {
                    return None;
                }
                chance_mult *= proj_mult;
            }
            // Resists
            if let Some(resist) = ospec.resist {
                let resist_mult = match resist {
                    REffectResist::Standard => {
                        funcs::get_effect_default_resist_mult(ctx, calc, projector_uid, effect, projectee_uid)
                    }
                    REffectResist::AttrRef(resist_ref_attr_rid) => funcs::get_referenced_resist_mult(
                        ctx,
                        calc,
                        &AttrSpec::new(projector_uid, resist_ref_attr_rid),
                        projectee_uid,
                    ),
                };
                match resist_mult {
                    Some(PValue::ZERO) => return None,
                    Some(resist_mult) => str_mult *= resist_mult,
                    None => (),
                }
            }
        }
        Some(Self {
            base_output,
            str_mult,
            instance_limit,
            chance_mult: process_mult(chance_mult),
        })
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
    pub(super) fn try_make<BG>(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        ospec: &REffectProjOpcSpec<BG>,
    ) -> Option<Self>
    where
        BG: NEffectOutputGetter,
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
pub(super) struct ProjConverterRegular<'sc1, 'sc2, 'calc, 'ospec, 'ip, BG, T>
where
    BG: NEffectOutputGetter,
    T: Copy,
{
    pub(super) ctx: SvcCtx<'sc1, 'sc2>,
    pub(super) calc: &'calc mut Calc,
    pub(super) projector_uid: UItemId,
    pub(super) ospec: &'ospec REffectProjOpcSpec<BG>,
    pub(super) inv_proj: &'ip AggrProjInvData<T>,
}
impl<'sc1, 'sc2, 'calc, 'ospec, 'ip, BG, T> ProjConverterRegular<'sc1, 'sc2, 'calc, 'ospec, 'ip, BG, T>
where
    BG: NEffectOutputGetter,
    T: Copy,
{
    pub(super) fn new(
        ctx: SvcCtx<'sc1, 'sc2>,
        calc: &'calc mut Calc,
        projector_uid: UItemId,
        ospec: &'ospec REffectProjOpcSpec<BG>,
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
impl<BG, T> LibConverter<CycleDataFull, AggrPartDataTail<T>> for ProjConverterRegular<'_, '_, '_, '_, '_, BG, T>
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataTail<T> {
        let output = get_proj_regular_output(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            self.inv_proj,
            input.active.chargedness,
        );
        let main_duration = input.get_main_duration();
        let tail_duration = output.get_completion_duration() - main_duration;
        let tail_duration = match tail_duration > Value::ZERO {
            true => Some(PValue::from_value_unchecked(tail_duration)),
            false => None,
        };
        AggrPartDataTail {
            cycle_main_duration: main_duration,
            cycle_tail_duration: tail_duration,
            output,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_proj_regular_output<BG, T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: &AggrProjInvData<T>,
    chargedness: Option<UnitInterval>,
) -> Output<T>
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
{
    let mut output = inv_proj.base_output;
    let mut str_mult = inv_proj.str_mult;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargedness
        && let Some(charge_mult) = charge_mult_getter.get(ctx, calc, item_uid, chargedness)
    {
        str_mult *= charge_mult;
    }
    if str_mult != PValue::ONE {
        output.instance_mul_assign(str_mult);
    }
    // Limit
    if let Some(limit) = inv_proj.instance_limit {
        output.instance_limit(limit);
    }
    output
}

pub(super) fn get_proj_spool_part_str_mult<BG, T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: &AggrProjInvData<T>,
    chargedness: Option<UnitInterval>,
) -> PValue
where
    BG: NEffectOutputGetter,
    T: Copy,
{
    let mut str_mult = inv_proj.str_mult;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargedness
        && let Some(charge_mult) = charge_mult_getter.get(ctx, calc, item_uid, chargedness)
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
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
{
    let mut output = inv_proj.base_output;
    // Spool
    str_mult *= PValue::from_value_clamped(Value::ONE + spool_extra_mult);
    if str_mult != PValue::ONE {
        output.instance_mul_assign(str_mult);
    }
    // Limit
    if let Some(instance_limit) = inv_proj.instance_limit {
        output.instance_limit(instance_limit);
    }
    output
}

fn process_mult(mult: PValue) -> Option<PValue> {
    match mult {
        PValue::ONE => None,
        v => Some(v),
    }
}
