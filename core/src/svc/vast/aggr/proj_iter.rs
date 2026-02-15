use super::{
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverter, get_proj_regular_output, get_proj_spool_part_str_mult,
    },
    shared_iter::{AggrIter, AggrPartDataRegular, AggrPartDataSpool},
    traits::{InstanceDuration, LimitInstance},
};
use crate::{
    num::PValue,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
    util::LibConverter,
};

// Projected effects, iterator over cycles (cycle time + instance iter)
pub(in crate::svc::vast) fn aggr_proj_iter<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
) -> Option<AggrIter<T>>
where
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
{
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid)?;
    let aggr_iter = match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        // Some(inv_spool) => aggr_spool(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool),
        Some(inv_spool) => return None,
        None => aggr_regular(ctx, calc, projector_uid, cseq, ospec, inv_proj),
    };
    Some(aggr_iter)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-spool
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: AggrProjInvData<T>,
) -> AggrIter<T>
where
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
{
    let mut converter = ProjConverter::new(ctx, calc, projector_uid, ospec, &inv_proj);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    AggrIter::new(cseq_conv.iter_cycles())
}

impl<T> LibConverter<CycleDataFull, AggrPartDataRegular<T>> for ProjConverter<'_, '_, '_, '_, '_, T>
where
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataRegular<T> {
        let output = get_proj_regular_output(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            &self.inv_proj,
            input.chargedness,
        );
        AggrPartDataRegular {
            cycle_duration: input.duration,
            output,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> LibConverter<CycleDataFull, AggrPartDataSpool> for ProjConverter<'_, '_, '_, '_, '_, T>
where
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataSpool {
        AggrPartDataSpool {
            cycle_duration: input.duration,
            interrupt: input.interrupt.is_some(),
            str_mult: get_proj_spool_part_str_mult(
                self.ctx,
                self.calc,
                self.projector_uid,
                self.ospec,
                self.inv_proj,
                input.chargedness,
            ),
        }
    }
}

// fn process_single_spool<T>(
//     ctx: SvcCtx,
//     calc: &mut Calc,
//     projector_uid: UItemId,
//     ospec: &REffectProjOpcSpec<T>,
//     inv_proj: &AggrProjInvData<T>,
//     inv_spool: &AggrSpoolInvData,
//     cycle_data: CycleDataFull,
//     uninterrupted_cycles: &mut Count,
// ) where
//     T: Copy + InstanceDuration + LimitInstance,
// {
//     if *time < Value::ZERO {
//         return;
//     }
//     let cycle_completion_duration = cycle_data
//         .duration
//         .max(inv_proj.output.get_completion_duration())
//         .into_value();
//     let charge_mult = calc_charge_mult(ctx, calc, projector_uid, ospec.charge_mult,
// cycle_data.chargedness);     let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
//     let cycle_output = get_proj_output_spool(inv_proj, charge_mult, cycle_spool);
//     match *time >= cycle_completion_duration {
//         true => accum.add_instance(
//             cycle_output.get_instance(),
//             inv_proj.chance_mult,
//             cycle_output.get_instance_count(),
//         ),
//         false => process_incomplete_cycle(accum, *time, &cycle_output, inv_proj.chance_mult),
//     }
//     *time -= cycle_data.duration;
//     match cycle_data.interrupt {
//         Some(_) => *uninterrupted_cycles = Count::ZERO,
//         None => *uninterrupted_cycles += Count::ONE,
//     }
// }

// fn process_limited_spool<T>(
//     ctx: SvcCtx,
//     calc: &mut Calc,
//     projector_uid: UItemId,
//     ospec: &REffectProjOpcSpec<T>,
//     inv_proj: &AggrProjInvData<T>,
//     inv_spool: &AggrSpoolInvData,
//     cycle_data: CycleDataFull,
//     uninterrupted_cycles: &mut Count,
//     mut repeat_limit: Count,
// ) where
//     T: Copy + InstanceDuration + LimitInstance,
// {
//     let cycle_tail_duration =
//         PValue::from_value_clamped(inv_proj.output.get_completion_duration() -
// cycle_data.duration);     let cycle_completion_duration = (cycle_data.duration +
// cycle_tail_duration).into_value();     let charge_mult = calc_charge_mult(ctx, calc,
// projector_uid, ospec.charge_mult, cycle_data.chargedness);     while *time >= Value::ZERO &&
// repeat_limit > Count::ZERO {         if cycle_data.interrupt.is_some() && *uninterrupted_cycles
// == Count::ZERO {             // Shortcut #1: we're at 0 spool and can't spool for the rest of the
// sequence             let cycle_output = get_proj_output_spool(inv_proj, charge_mult,
// Value::ZERO);             let full_repeats =
//                 repeat_limit.min(get_full_repeats_count(*time, cycle_data.duration,
// cycle_tail_duration));             // Full repeats
//             if full_repeats > Count::ZERO {
//                 repeat_limit -= full_repeats;
//                 accum.add_instance(
//                     cycle_output.get_instance(),
//                     inv_proj.chance_mult,
//                     cycle_output.get_instance_count() * full_repeats,
//                 );
//                 *time -= cycle_data.duration * full_repeats.into_pvalue();
//             }
//             // Partial repeats
//             while *time >= Value::ZERO && repeat_limit > Count::ZERO {
//                 repeat_limit -= Count::ONE;
//                 process_incomplete_cycle(accum, *time, &cycle_output, inv_proj.chance_mult);
//                 *time -= cycle_data.duration;
//             }
//             return;
//         } else if cycle_data.interrupt.is_none() && *uninterrupted_cycles >=
// inv_spool.cycles_to_max {             // Shortcut #2: we're at max spool and sequence is not
// interruptable             let cycle_output = get_proj_output_spool(inv_proj, charge_mult,
// inv_spool.max);             let full_repeats =
//                 repeat_limit.min(get_full_repeats_count(*time, cycle_data.duration,
// cycle_tail_duration));             // Full repeats
//             if full_repeats > Count::ZERO {
//                 repeat_limit -= full_repeats;
//                 *uninterrupted_cycles += full_repeats;
//                 accum.add_instance(
//                     cycle_output.get_instance(),
//                     inv_proj.chance_mult,
//                     cycle_output.get_instance_count() * full_repeats,
//                 );
//                 *time -= cycle_data.duration * full_repeats.into_pvalue();
//             }
//             // Partial repeats
//             while *time >= Value::ZERO && repeat_limit > Count::ZERO {
//                 repeat_limit -= Count::ONE;
//                 *uninterrupted_cycles += Count::ONE;
//                 process_incomplete_cycle(accum, *time, &cycle_output, inv_proj.chance_mult);
//                 *time -= cycle_data.duration;
//             }
//             return;
//         } else {
//             let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
//             let cycle_output = get_proj_output_spool(inv_proj, charge_mult, cycle_spool);
//             match *time >= cycle_completion_duration {
//                 true => accum.add_instance(
//                     cycle_output.get_instance(),
//                     inv_proj.chance_mult,
//                     cycle_output.get_instance_count(),
//                 ),
//                 false => process_incomplete_cycle(accum, *time, &cycle_output,
// inv_proj.chance_mult),             }
//             *time -= cycle_data.duration;
//             match cycle_data.interrupt {
//                 Some(_) => *uninterrupted_cycles = Count::ZERO,
//                 None => *uninterrupted_cycles += Count::ONE,
//             }
//             repeat_limit -= Count::ONE;
//         }
//     }
// }

// fn process_infinite_spool<T>(
//     ctx: SvcCtx,
//     calc: &mut Calc,
//     projector_uid: UItemId,
//     ospec: &REffectProjOpcSpec<T>,
//     inv_proj: &AggrProjInvData<T>,
//     inv_spool: &AggrSpoolInvData,
//     cycle_data: CycleDataFull,
//     uninterrupted_cycles: &mut Count,
// ) where
//     T: Copy + InstanceDuration + LimitInstance,
// {
//     if *time < Value::ZERO {
//         return;
//     }
//     let cycle_tail_duration =
//         PValue::from_value_clamped(inv_proj.output.get_completion_duration() -
// cycle_data.duration);     let cycle_completion_duration = (cycle_data.duration +
// cycle_tail_duration).into_value();     let charge_mult = calc_charge_mult(ctx, calc,
// projector_uid, ospec.charge_mult, cycle_data.chargedness);     while *time >= Value::ZERO {
//         if cycle_data.interrupt.is_some() && *uninterrupted_cycles == Count::ZERO {
//             // Shortcut #1: we're at 0 spool and can't spool for the rest of the sequence
//             let cycle_output = get_proj_output_spool(inv_proj, charge_mult, Value::ZERO);
//             let full_repeats = get_full_repeats_count(*time, cycle_data.duration,
// cycle_tail_duration);             // Full repeats
//             accum.add_instance(
//                 cycle_output.get_instance(),
//                 inv_proj.chance_mult,
//                 cycle_output.get_instance_count() * full_repeats,
//             );
//             *time -= cycle_data.duration * full_repeats.into_pvalue();
//             // Partial repeats
//             while *time >= Value::ZERO {
//                 process_incomplete_cycle(accum, *time, &cycle_output, inv_proj.chance_mult);
//                 *time -= cycle_data.duration;
//             }
//             return;
//         } else if cycle_data.interrupt.is_none() && *uninterrupted_cycles >=
// inv_spool.cycles_to_max {             // Shortcut #2: we're at max spool and sequence is not
// interruptable             let cycle_output = get_proj_output_spool(inv_proj, charge_mult,
// inv_spool.max);             let full_repeats = get_full_repeats_count(*time, cycle_data.duration,
// cycle_tail_duration);             // Full repeats
//             *uninterrupted_cycles += full_repeats;
//             accum.add_instance(
//                 cycle_output.get_instance(),
//                 inv_proj.chance_mult,
//                 cycle_output.get_instance_count() * full_repeats,
//             );
//             *time -= cycle_data.duration * full_repeats.into_pvalue();
//             // Partial repeats
//             while *time >= Value::ZERO {
//                 *uninterrupted_cycles += Count::ONE;
//                 process_incomplete_cycle(accum, *time, &cycle_output, inv_proj.chance_mult);
//                 *time -= cycle_data.duration;
//             }
//             return;
//         } else {
//             // Regular cycle-by-cycle processing
//             let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
//             let cycle_output = get_proj_output_spool(inv_proj, charge_mult, cycle_spool);
//             match *time >= cycle_completion_duration {
//                 true => accum.add_instance(
//                     cycle_output.get_instance(),
//                     inv_proj.chance_mult,
//                     cycle_output.get_instance_count(),
//                 ),
//                 false => process_incomplete_cycle(accum, *time, &cycle_output,
// inv_proj.chance_mult),             }
//             *time -= cycle_data.duration;
//             match cycle_data.interrupt {
//                 Some(_) => *uninterrupted_cycles = Count::ZERO,
//                 None => *uninterrupted_cycles += Count::ONE,
//             }
//         }
//     }
// }
