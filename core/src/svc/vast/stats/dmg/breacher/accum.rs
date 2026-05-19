// In this module, there are two different accumulators for breacher damage:
//
// - regular (non-applied): provides stats in a form of "absolute limit + relative limit" pair, but
//   this solution is approximate. This approximation breaks when there are e.g. 2 breachers which
//   deal [0 abs + 100% rel] and [1000 abs + 0% rel] during the same tick, the accumulator will
//   aggregate it into [1000 abs + 100% rel], which is not how it is supposed to work;
// - applied: this accumulator takes extra context and provides accurate number of applied damage.
//
// They are two different entities exactly because it is impossible to build the second one
// (accurate solution) on top of the first one (approximate solution).

use std::collections::hash_map::Entry;

use super::{
    conv::cseq_to_ticks,
    shared::{duration_to_ticks_floor, ticks_to_duration},
    ticks::AggrBreacherTicks,
};
use crate::{
    def::SERVER_TICK_HZ,
    misc::EffectSpec,
    nd::{NEffectBreacherOutputGetter, NEffectOutputGetter},
    num::{Count, PValue, UnitInterval, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
        vast::StatDmgEntryBreacher,
    },
    ud::UItemId,
    util::RMap,
};

const DAY_TICKS: Count = Count::from_u32(24 * 60 * 60 * SERVER_TICK_HZ as u32);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular accumulator
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct BreacherData {
    absolute_max: PValue,
    relative_max: UnitInterval,
    ticks: AggrBreacherTicks,
}

pub(in crate::svc::vast::stats::dmg) struct BreacherAccum {
    data: RMap<BreacherData, Count>,
}
impl BreacherAccum {
    pub(in crate::svc::vast) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(in crate::svc::vast) fn add(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
        ospec: &REffectProjOpcSpec<NEffectBreacherOutputGetter>,
    ) {
        // Base output
        let Some(output) = ospec.base.get(ctx, calc, item_uid, effect, ()) else {
            return;
        };
        // Cycle sequence conversion
        let Some(ticks) = cseq_to_ticks(cseq, output) else {
            return;
        };
        let accum_entry = BreacherData {
            absolute_max: output.get_instance().absolute_max,
            relative_max: output.get_instance().relative_max,
            ticks,
        };
        match self.data.entry(accum_entry) {
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                entry.insert(accum_entry.ticks.get_loop_len());
            }
        }
    }
    pub(in crate::svc::vast) fn get_dps(&self) -> StatDmgEntryBreacher {
        if self.data.is_empty() {
            return StatDmgEntryBreacher {
                absolute_max: PValue::ZERO,
                relative_max: PValue::ZERO,
            };
        };
        let max_dmg_abs = self.data.keys().map(|v| v.absolute_max).max().unwrap();
        let max_dmg_rel = self.data.keys().map(|v| v.relative_max).max().unwrap();
        // Shortcut - if breacher with max damage is applying its damage without downtime, no
        // complex calcs needed
        if self
            .data
            .keys()
            .any(|v| v.absolute_max >= max_dmg_abs && matches!(v.ticks, AggrBreacherTicks::Infinite(_)))
            && self
                .data
                .keys()
                .any(|v| v.relative_max >= max_dmg_rel && matches!(v.ticks, AggrBreacherTicks::Infinite(_)))
        {
            return StatDmgEntryBreacher {
                absolute_max: max_dmg_abs * PValue::SERVER_TICK_HZ,
                relative_max: max_dmg_rel.into_pvalue() * PValue::SERVER_TICK_HZ,
            };
        }
        // General solution is go tick-to-tick until items are looped, pick max for each tick, and
        // then calculate average. Total count of ticks we consider is limited by 1 day to avoid
        // excessively cpu-heavy configurations
        let loop_tick_count = Count::from_u32(
            self.data
                .values()
                .map(|v| v.into_u32())
                .reduce(num_integer::lcm)
                .unwrap(),
        )
        .min(DAY_TICKS);
        let max_delay_tick = self.data.keys().map(|v| v.ticks.get_initial_delay()).max().unwrap();
        let (loop_dmg_abs, loop_dmg_rel) =
            self.get_dmg_for_tick_range(max_delay_tick, max_delay_tick + loop_tick_count);
        StatDmgEntryBreacher {
            absolute_max: loop_dmg_abs / loop_tick_count.into_pvalue() * PValue::SERVER_TICK_HZ,
            relative_max: loop_dmg_rel / loop_tick_count.into_pvalue() * PValue::SERVER_TICK_HZ,
        }
    }
    pub(in crate::svc::vast) fn get_dps_by_time(&self, time: PValue) -> StatDmgEntryBreacher {
        let mut dmg = StatDmgEntryBreacher {
            absolute_max: PValue::ZERO,
            relative_max: PValue::ZERO,
        };
        if self.data.is_empty() {
            return dmg;
        };
        // The tick after the last tick which should be included in stats
        let stop_tick = duration_to_ticks_floor(time) + Count::ONE;
        // How many ticks does a loop take
        let loop_tick_count = Count::from_u32(
            self.data
                .values()
                .map(|v| v.into_u32())
                .reduce(num_integer::lcm)
                .unwrap(),
        );
        let max_delay_tick = self.data.keys().map(|v| v.ticks.get_initial_delay()).max().unwrap();
        // Loops start only after longest starting delay is done
        let full_loops = match stop_tick > max_delay_tick {
            true => (stop_tick - max_delay_tick) / loop_tick_count,
            false => Count::ZERO,
        };
        // Record damage done before loops start
        let (early_dmg_abs, early_dmg_rel) = self.get_dmg_for_tick_range(Count::ZERO, max_delay_tick.min(stop_tick));
        dmg.absolute_max += early_dmg_abs;
        dmg.relative_max += early_dmg_rel;
        // Record damage done during loops
        if full_loops > Count::ZERO {
            let (loop_dmg_abs, loop_dmg_rel) =
                self.get_dmg_for_tick_range(max_delay_tick, max_delay_tick + loop_tick_count);
            dmg.absolute_max += loop_dmg_abs * full_loops.into_pvalue();
            dmg.relative_max += loop_dmg_rel * full_loops.into_pvalue();
        }
        // Record damage done after loops
        let loops_done_tick = max_delay_tick + loop_tick_count * full_loops;
        if stop_tick > loops_done_tick {
            let (late_dmg_abs, late_dmg_rel) = self.get_dmg_for_tick_range(loops_done_tick, stop_tick);
            dmg.absolute_max += late_dmg_abs;
            dmg.relative_max += late_dmg_rel;
        }
        StatDmgEntryBreacher {
            absolute_max: dmg.absolute_max / time * PValue::SERVER_TICK_HZ,
            relative_max: dmg.relative_max / time * PValue::SERVER_TICK_HZ,
        }
    }
    fn get_dmg_for_tick_range(&self, start_tick: Count, end_tick: Count) -> (PValue, PValue) {
        let mut dmg_abs = PValue::ZERO;
        let mut dmg_rel = PValue::ZERO;
        for tick in start_tick..end_tick {
            let mut tick_max_abs = PValue::ZERO;
            let mut tick_max_rel = UnitInterval::ZERO;
            for breacher in self.data.keys() {
                if breacher.ticks.is_applied_on_tick(tick) {
                    tick_max_abs = tick_max_abs.max(breacher.absolute_max);
                    tick_max_rel = tick_max_rel.max(breacher.relative_max);
                }
            }
            dmg_abs += tick_max_abs;
            dmg_rel += tick_max_rel.into_pvalue();
        }
        (dmg_abs, dmg_rel)
    }
    pub(in crate::svc::vast) fn get_volley(&self) -> StatDmgEntryBreacher {
        let mut volley = StatDmgEntryBreacher {
            absolute_max: PValue::ZERO,
            relative_max: PValue::ZERO,
        };
        if self.data.is_empty() {
            return volley;
        };
        for entry in self.data.keys() {
            volley.absolute_max = volley.absolute_max.max(entry.absolute_max);
            volley.relative_max = volley.relative_max.max(entry.relative_max.into_pvalue());
        }
        volley
    }
    pub(in crate::svc::vast) fn get_volley_by_time(&self, time: PValue) -> StatDmgEntryBreacher {
        let mut volley = StatDmgEntryBreacher {
            absolute_max: PValue::ZERO,
            relative_max: PValue::ZERO,
        };
        if self.data.is_empty() {
            return volley;
        };
        for entry in self.data.keys() {
            if time < ticks_to_duration(entry.ticks.get_initial_delay()) {
                continue;
            }
            volley.absolute_max = volley.absolute_max.max(entry.absolute_max);
            volley.relative_max = volley.relative_max.max(entry.relative_max.into_pvalue());
        }
        volley
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Applied accumulator
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AppliedBreacherData {
    dmg: PValue,
    ticks: AggrBreacherTicks,
}

pub(in crate::svc::vast::stats::dmg) struct AppliedBreacherAccum {
    data: RMap<AppliedBreacherData, Count>,
}
impl AppliedBreacherAccum {
    pub(in crate::svc::vast) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(in crate::svc::vast) fn add(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
        ospec: &REffectProjOpcSpec<NEffectBreacherOutputGetter>,
        projectee_uid: UItemId,
    ) {
        // Base output
        let Some(output) = ospec.base.get(ctx, calc, item_uid, effect, ()) else {
            return;
        };
        // Applied output against target HP
        let projectee_hp = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            projectee_uid,
            ctx.ac().shield_capacity,
            Value::ZERO,
        )) + PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            projectee_uid,
            ctx.ac().armor_hp,
            Value::ZERO,
        )) + PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            projectee_uid,
            ctx.ac().hp,
            Value::ZERO,
        ));
        let mut applied = output
            .get_instance()
            .absolute_max
            .min(output.get_instance().relative_max.into_pvalue() * projectee_hp);
        if applied == PValue::ZERO {
            return;
        }
        // Projection multiplication
        if let Some(proj_mult_getter) = ospec.proj_mult_chance {
            let proj_data =
                ctx.eff_projs
                    .get_or_make_proj_data(ctx.u_data, EffectSpec::new(item_uid, effect.rid), projectee_uid);
            applied *= proj_mult_getter.get_mult(ctx, calc, item_uid, effect, projectee_uid, proj_data);
            if applied == PValue::ZERO {
                return;
            }
        };
        // Cycle sequence conversion
        let Some(ticks) = cseq_to_ticks(cseq, output) else {
            return;
        };
        let accum_entry = AppliedBreacherData { dmg: applied, ticks };
        match self.data.entry(accum_entry) {
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                entry.insert(accum_entry.ticks.get_loop_len());
            }
        }
    }
    pub(in crate::svc::vast) fn get_dps(&self) -> PValue {
        if self.data.is_empty() {
            return PValue::ZERO;
        };
        let max_dmg = self.data.keys().map(|v| v.dmg).max().unwrap();
        // Shortcut - if breacher with max damage is applying its damage without downtime, no
        // complex calcs needed
        for accum_entry in self.data.keys() {
            if accum_entry.dmg >= max_dmg && matches!(accum_entry.ticks, AggrBreacherTicks::Infinite(_)) {
                return accum_entry.dmg * PValue::SERVER_TICK_HZ;
            }
        }
        // General solution is go tick-to-tick until items are looped, pick max for each tick, and
        // then calculate average. Total count of ticks we consider is limited by 1 day to avoid
        // excessively cpu-heavy configurations
        let loop_tick_count = Count::from_u32(
            self.data
                .values()
                .map(|v| v.into_u32())
                .reduce(num_integer::lcm)
                .unwrap(),
        )
        .min(DAY_TICKS);
        let max_delay_tick = self.data.keys().map(|v| v.ticks.get_initial_delay()).max().unwrap();
        let loop_dmg = self.get_dmg_for_tick_range(max_delay_tick, max_delay_tick + loop_tick_count);
        loop_dmg / loop_tick_count.into_pvalue() * PValue::SERVER_TICK_HZ
    }
    pub(in crate::svc::vast) fn get_dps_by_time(&self, time: PValue) -> PValue {
        let mut total_dmg = PValue::ZERO;
        if self.data.is_empty() {
            return total_dmg;
        };
        // The tick after the last tick which should be included in stats
        let stop_tick = duration_to_ticks_floor(time) + Count::ONE;
        // How many ticks does a loop take
        let loop_tick_count = Count::from_u32(
            self.data
                .values()
                .map(|v| v.into_u32())
                .reduce(num_integer::lcm)
                .unwrap(),
        );
        let max_delay_tick = self.data.keys().map(|v| v.ticks.get_initial_delay()).max().unwrap();
        // Loops start only after longest starting delay is done
        let full_loops = match stop_tick > max_delay_tick {
            true => (stop_tick - max_delay_tick) / loop_tick_count,
            false => Count::ZERO,
        };
        // Record damage done before loops start
        total_dmg += self.get_dmg_for_tick_range(Count::ZERO, max_delay_tick.min(stop_tick));
        // Record damage done during loops
        if full_loops > Count::ZERO {
            let loop_dmg = self.get_dmg_for_tick_range(max_delay_tick, max_delay_tick + loop_tick_count);
            total_dmg += loop_dmg * full_loops.into_pvalue();
        }
        // Record damage done after loops
        let loops_done_tick = max_delay_tick + loop_tick_count * full_loops;
        if stop_tick > loops_done_tick {
            total_dmg += self.get_dmg_for_tick_range(loops_done_tick, stop_tick);
        }
        total_dmg / time
    }
    fn get_dmg_for_tick_range(&self, start_tick: Count, end_tick: Count) -> PValue {
        let mut total_dmg = PValue::ZERO;
        for tick in start_tick..end_tick {
            let mut tick_max_dmg = PValue::ZERO;
            for breacher in self.data.keys() {
                if breacher.ticks.is_applied_on_tick(tick) {
                    tick_max_dmg = tick_max_dmg.max(breacher.dmg);
                }
            }
            total_dmg += tick_max_dmg;
        }
        total_dmg
    }
    pub(in crate::svc::vast) fn get_volley(&self) -> PValue {
        self.data.keys().map(|v| v.dmg).max().unwrap_or(PValue::ZERO)
    }
    pub(in crate::svc::vast) fn get_volley_by_time(&self, time: PValue) -> PValue {
        self.data
            .keys()
            .filter_map(|v| match time >= ticks_to_duration(v.ticks.get_initial_delay()) {
                true => Some(v.dmg),
                false => None,
            })
            .max()
            .unwrap_or(PValue::ZERO)
    }
}
