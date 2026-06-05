use super::{
    accum::SeqInstanceAccum,
    shared::{
        AggrHardDtSimple, AggrPartData, AggrPartDataSpool, AggrPartDataSpoolTail, AggrPartDataTail,
        get_cycle_tail_duration, get_item_ship_limit, get_tailed_cycle_full_repeat_count,
    },
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    misc::{AttrSpec, EffectSpec},
    nd::NEffectOutputGetter,
    num::{Count, PValue, UnitInterval, Value},
    rd::{REffect, REffectProjOpcSpec, REffectResist},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqLoopLimSin, CycleDataFull, GetMainDuration},
        funcs,
        output::Output,
    },
    ud::UItemId,
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// General data which stays the same through projected effect cycling
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(super) struct AggrProjInvData<I> {
    base_output: Output<I>,
    str_mult: PValue,
    instance_limit: Option<PValue>,
    pub(super) chance_mult: Option<PValue>,
}
impl<I> AggrProjInvData<I> {
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
        I: std::ops::MulAssign<PValue> + HasImpact,
        BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
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
pub(super) struct ProjConverter<'sc1, 'sc2, 'calc, 'ospec, 'ip, BG, I>
where
    BG: NEffectOutputGetter,
{
    pub(super) ctx: SvcCtx<'sc1, 'sc2>,
    pub(super) calc: &'calc mut Calc,
    pub(super) projector_uid: UItemId,
    pub(super) ospec: &'ospec REffectProjOpcSpec<BG>,
    pub(super) inv_proj: &'ip AggrProjInvData<I>,
}
impl<'sc1, 'sc2, 'calc, 'ospec, 'ip, BG, I> ProjConverter<'sc1, 'sc2, 'calc, 'ospec, 'ip, BG, I>
where
    BG: NEffectOutputGetter,
{
    pub(super) fn new(
        ctx: SvcCtx<'sc1, 'sc2>,
        calc: &'calc mut Calc,
        projector_uid: UItemId,
        ospec: &'ospec REffectProjOpcSpec<BG>,
        inv_proj: &'ip AggrProjInvData<I>,
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
impl<BG, I> LibConverter<CycleDataFull, AggrPartData<I>> for ProjConverter<'_, '_, '_, '_, '_, BG, I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartData<I> {
        let output = get_proj_regular_output(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            self.inv_proj,
            input.active.chargedness,
        );
        AggrPartData {
            cycle_main_duration: input.get_main_duration(),
            output,
        }
    }
}
impl<BG, I> LibConverter<CycleDataFull, AggrPartDataTail<I>> for ProjConverter<'_, '_, '_, '_, '_, BG, I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataTail<I> {
        let output = get_proj_regular_output(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            self.inv_proj,
            input.active.chargedness,
        );
        let cycle_main_duration = input.get_main_duration();
        AggrPartDataTail {
            cycle_main_duration,
            cycle_tail_duration: get_cycle_tail_duration(cycle_main_duration, output.get_completion_duration()),
            output,
        }
    }
}
impl<BG, I> LibConverter<CycleDataFull, AggrPartDataSpool> for ProjConverter<'_, '_, '_, '_, '_, BG, I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataSpool {
        let str_mult = get_proj_spool_part_str_mult(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            self.inv_proj,
            input.active.chargedness,
        );
        AggrPartDataSpool {
            cycle_main_duration: input.get_main_duration(),
            soft_dt: input.soft_dt.is_some(),
            str_mult,
        }
    }
}
impl<BG, I> LibConverter<CycleDataFull, AggrPartDataSpoolTail> for ProjConverter<'_, '_, '_, '_, '_, BG, I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataSpoolTail {
        let str_mult = get_proj_spool_part_str_mult(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            self.inv_proj,
            input.active.chargedness,
        );
        let cycle_main_duration = input.get_main_duration();
        let output_completion_duration = self.inv_proj.base_output.get_completion_duration();
        AggrPartDataSpoolTail {
            cycle_main_duration,
            cycle_completion_duration: cycle_main_duration.max(output_completion_duration).into_value(),
            cycle_tail_duration: get_cycle_tail_duration(cycle_main_duration, output_completion_duration),
            soft_dt: input.soft_dt.is_some(),
            str_mult,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cseq/part/cycle processing functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn process_single_spool<I, IA>(
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    part_data: AggrPartDataSpoolTail,
    accum: &mut IA,
    time: &mut Value,
    uninterrupted_cycles: &mut Count,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    if *time < Value::ZERO {
        return;
    }
    let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
    let cycle_output = get_proj_spool_cycle_output(inv_proj, part_data.str_mult, cycle_spool);
    match *time >= part_data.cycle_completion_duration {
        true => accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE),
        false => accum.add_output_time_limited(&cycle_output, inv_proj.chance_mult, Count::ONE, *time),
    }
    *time -= part_data.cycle_main_duration;
    match part_data.soft_dt {
        true => *uninterrupted_cycles = Count::ZERO,
        false => *uninterrupted_cycles += Count::ONE,
    }
}

pub(super) fn process_limited_spool<I, IA>(
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    part_data: AggrPartDataSpoolTail,
    accum: &mut IA,
    time: &mut Value,
    uninterrupted_cycles: &mut Count,
    mut repeat_limit: Count,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    while *time >= Value::ZERO && repeat_limit > Count::ZERO {
        // Case when spool multiplier does not change for the rest of cycles of current part
        let stable_spool = match part_data.soft_dt {
            // Current cycle is at 0 spool, and we have an interrupt every cycle
            true if *uninterrupted_cycles == Count::ZERO => Some(Value::ZERO),
            // Current cycle is at max spool, and we have no interrupts in cycles of current
            // part
            false if *uninterrupted_cycles >= inv_spool.cycles_to_max => Some(inv_spool.max),
            _ => None,
        };
        if let Some(stable_spool) = stable_spool {
            let full_repeat_count = repeat_limit.min(get_tailed_cycle_full_repeat_count(
                *time,
                part_data.cycle_main_duration,
                part_data.cycle_tail_duration,
            ));
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_data.str_mult, stable_spool);
            // Full repeats
            if full_repeat_count > Count::ZERO {
                accum.add_output_full(&cycle_output, inv_proj.chance_mult, full_repeat_count);
                *time -= part_data.cycle_main_duration * full_repeat_count.into_pvalue();
                if !part_data.soft_dt {
                    *uninterrupted_cycles += full_repeat_count;
                }
                repeat_limit -= full_repeat_count;
            }
            // Partial repeats
            while *time >= Value::ZERO && repeat_limit > Count::ZERO {
                accum.add_output_time_limited(&cycle_output, inv_proj.chance_mult, Count::ONE, *time);
                *time -= part_data.cycle_main_duration;
                if !part_data.soft_dt {
                    *uninterrupted_cycles += Count::ONE;
                }
                repeat_limit -= Count::ONE;
            }
            return;
        }
        let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
        let cycle_output = get_proj_spool_cycle_output(inv_proj, part_data.str_mult, cycle_spool);
        match *time >= part_data.cycle_completion_duration {
            true => accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE),
            false => accum.add_output_time_limited(&cycle_output, inv_proj.chance_mult, Count::ONE, *time),
        }
        *time -= part_data.cycle_main_duration;
        match part_data.soft_dt {
            true => *uninterrupted_cycles = Count::ZERO,
            false => *uninterrupted_cycles += Count::ONE,
        }
        repeat_limit -= Count::ONE;
    }
}

pub(super) fn process_infinite_spool<I, IA>(
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    part_data: AggrPartDataSpoolTail,
    accum: &mut IA,
    time: &mut Value,
    uninterrupted_cycles: &mut Count,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    if *time < Value::ZERO {
        return;
    }
    while *time >= Value::ZERO {
        // Case when spool multiplier does not change for the rest of cycles of current part
        let stable_spool = match part_data.soft_dt {
            // Current cycle is at 0 spool, and we have an interrupt every cycle
            true if *uninterrupted_cycles == Count::ZERO => Some(Value::ZERO),
            // Current cycle is at max spool, and we have no interrupts in cycles of current
            // part
            false if *uninterrupted_cycles >= inv_spool.cycles_to_max => Some(inv_spool.max),
            _ => None,
        };
        if let Some(stable_spool) = stable_spool {
            let full_repeat_count =
                get_tailed_cycle_full_repeat_count(*time, part_data.cycle_main_duration, part_data.cycle_tail_duration);
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_data.str_mult, stable_spool);
            // Full repeats
            if full_repeat_count > Count::ZERO {
                accum.add_output_full(&cycle_output, inv_proj.chance_mult, full_repeat_count);
                *time -= part_data.cycle_main_duration * full_repeat_count.into_pvalue();
                if !part_data.soft_dt {
                    *uninterrupted_cycles += full_repeat_count;
                }
            }
            // Partial repeats
            while *time >= Value::ZERO {
                accum.add_output_time_limited(&cycle_output, inv_proj.chance_mult, Count::ONE, *time);
                *time -= part_data.cycle_main_duration;
                if !part_data.soft_dt {
                    *uninterrupted_cycles += Count::ONE;
                }
            }
            return;
        }
        // Regular cycle-by-cycle processing
        let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
        let cycle_output = get_proj_spool_cycle_output(inv_proj, part_data.str_mult, cycle_spool);
        match *time >= part_data.cycle_completion_duration {
            true => accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE),
            false => accum.add_output_time_limited(&cycle_output, inv_proj.chance_mult, Count::ONE, *time),
        }
        *time -= part_data.cycle_main_duration;
        match part_data.soft_dt {
            true => *uninterrupted_cycles = Count::ZERO,
            false => *uninterrupted_cycles += Count::ONE,
        }
    }
}

pub(super) fn process_output_of_spooling_lls_with_cutoff<I, IA>(
    cseq: &CSeqLoopLimSin<AggrPartDataSpoolTail, AggrHardDtSimple>,
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    accum: &mut IA,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    // Hard downtime resets uninterrupted cycles, so always start from 0
    let mut uninterrupted_cycles = Count::ZERO;
    process_limited_spool(
        inv_proj,
        inv_spool,
        cseq.p1_data,
        accum,
        &mut cseq.get_main_duration().into_value(),
        &mut uninterrupted_cycles,
        cseq.p1_repeat_count,
    );
    // Tracking time remaining after part 1 would be prone to float calculation errors. Instead,
    // pass active + soft downtime duration as soft limit, since after that hard downtime hits.
    process_single_spool(
        inv_proj,
        inv_spool,
        cseq.p2_data,
        accum,
        &mut cseq.p2_data.cycle_main_duration.into_value(),
        &mut uninterrupted_cycles,
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_proj_regular_output<BG, I>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: &AggrProjInvData<I>,
    chargedness: Option<UnitInterval>,
) -> Output<I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
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

pub(super) fn get_proj_spool_part_str_mult<BG, I>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: &AggrProjInvData<I>,
    chargedness: Option<UnitInterval>,
) -> PValue
where
    BG: NEffectOutputGetter,
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

pub(super) fn get_proj_spool_cycle_output<I>(
    inv_proj: &AggrProjInvData<I>,
    mut str_mult: PValue,
    spool_extra_mult: Value,
) -> Output<I>
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
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
