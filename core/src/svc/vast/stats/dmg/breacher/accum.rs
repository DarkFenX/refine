use std::collections::hash_map::Entry;

use super::{
    conv::cseq_to_ticks,
    shared::{duration_to_ticks_floor, ticks_to_duration},
    ticks::AggrBreacherTicks,
};
use crate::{
    def::SERVER_TICK_HZ,
    misc::{Breacher, EffectSpec},
    nd::NEffectProjOpcSpec,
    num::{Count, PValue, UnitInterval, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
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
        cseq: CycleSeq<CycleDataFull>,
        ospec: &NEffectProjOpcSpec<Breacher>,
    ) {
        // Base output
        let output = match (ospec.base)(ctx, calc, item_uid, effect) {
            Some(output) => output,
            None => return,
        };
        // Cycle sequence conversion
        let ticks = match cseq_to_ticks(cseq.convert_and_optimize(), output) {
            Some(ticks) => ticks,
            None => return,
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
    pub(in crate::svc::vast) fn get_volley(&self) -> Option<StatDmgEntryBreacher> {
        if self.data.is_empty() {
            return None;
        };
        let mut max_abs = PValue::ZERO;
        let mut max_rel = UnitInterval::ZERO;
        for entry in self.data.keys() {
            max_abs = max_abs.max(entry.absolute_max);
            max_rel = max_rel.max(entry.relative_max);
        }
        StatDmgEntryBreacher {
            absolute_max: max_abs,
            relative_max: max_rel.into_pvalue(),
        }
        .nullified()
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
        cseq: CycleSeq<CycleDataFull>,
        ospec: &NEffectProjOpcSpec<Breacher>,
        projectee_uid: UItemId,
    ) {
        // Base output
        let output = match (ospec.base)(ctx, calc, item_uid, effect) {
            Some(output) => output,
            None => return,
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
            applied *= proj_mult_getter(ctx, calc, item_uid, effect, projectee_uid, proj_data);
            if applied == PValue::ZERO {
                return;
            }
        };
        // Cycle sequence conversion
        let ticks = match cseq_to_ticks(cseq.convert_and_optimize(), output) {
            Some(ticks) => ticks,
            None => return,
        };
        let accum_entry = AppliedBreacherData { dmg: applied, ticks };
        match self.data.entry(accum_entry) {
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                entry.insert(accum_entry.ticks.get_loop_len());
            }
        }
    }
    pub(in crate::svc::vast) fn get_dps(&self) -> Option<PValue> {
        if self.data.is_empty() {
            return None;
        };
        let max_dmg = self.data.keys().map(|v| v.dmg).max()?;
        // Shortcut - if breacher with max damage is applying its damage without downtime, no
        // complex calcs needed
        for accum_entry in self.data.keys() {
            if accum_entry.dmg >= max_dmg && matches!(accum_entry.ticks, AggrBreacherTicks::Infinite(_)) {
                return Some(accum_entry.dmg * PValue::SERVER_TICK_HZ);
            }
        }
        // General solution is go tick-to-tick until items are looped, pick max for each tick, and
        // then calculate average. Total count of ticks we consider is limited by 1 day to avoid
        // excessively cpu-heavy configurations
        let total_ticks = Count::from_u32(
            self.data
                .values()
                .map(|v| v.into_u32())
                .reduce(num_integer::lcm)
                .unwrap(),
        )
        .min(DAY_TICKS);
        let max_initial_delay = self.data.keys().map(|v| v.ticks.get_initial_delay()).max().unwrap();
        let mut total_dmg = PValue::ZERO;
        total_dmg += self.add_applied_dmg_for_tick_range(max_initial_delay, max_initial_delay + total_ticks);
        match total_dmg {
            PValue::ZERO => None,
            n => Some(n / total_ticks.into_pvalue() * PValue::SERVER_TICK_HZ),
        }
    }
    pub(in crate::svc::vast) fn get_dps_by_time(&self, time: PValue) -> Option<PValue> {
        if self.data.is_empty() {
            return None;
        };
        // Last tick which should be included in stats
        let time_ticks = duration_to_ticks_floor(time);
        // How many ticks does a loop take
        let loop_ticks = Count::from_u32(
            self.data
                .values()
                .map(|v| v.into_u32())
                .reduce(num_integer::lcm)
                .unwrap(),
        );
        let max_initial_delay = self.data.keys().map(|v| v.ticks.get_initial_delay()).max().unwrap();
        // Loops start only after longest starting delay is done
        let full_loops = match time_ticks >= max_initial_delay {
            true => (time_ticks - max_initial_delay) / loop_ticks,
            false => Count::ZERO,
        };
        let mut total_dmg = PValue::ZERO;
        // Record damage done before loops start
        total_dmg += self.add_applied_dmg_for_tick_range(Count::ZERO, time_ticks.min(max_initial_delay));
        // Record damage done during loops
        if full_loops > Count::ZERO {
            let loop_dmg = self.add_applied_dmg_for_tick_range(max_initial_delay, max_initial_delay + loop_ticks);
            total_dmg += loop_dmg * full_loops.into_pvalue();
        }
        // Record damage done after loops
        let loops_done_tick = max_initial_delay + loop_ticks * full_loops;
        if time_ticks > loops_done_tick {
            total_dmg += self.add_applied_dmg_for_tick_range(loops_done_tick, time_ticks + Count::ONE);
        }
        match total_dmg {
            PValue::ZERO => None,
            n => Some(n / time),
        }
    }
    pub(in crate::svc::vast) fn get_volley(&self) -> Option<PValue> {
        if self.data.is_empty() {
            return None;
        };
        let max_dmg = self.data.keys().map(|v| v.dmg).max()?;
        match max_dmg {
            PValue::ZERO => None,
            n => Some(n),
        }
    }
    pub(in crate::svc::vast) fn get_volley_by_time(&self, time: PValue) -> Option<PValue> {
        if self.data.is_empty() {
            return None;
        };
        let max_dmg = self
            .data
            .keys()
            .filter_map(|v| match time >= ticks_to_duration(v.ticks.get_initial_delay()) {
                true => Some(v.dmg),
                false => None,
            })
            .max()?;
        match max_dmg {
            PValue::ZERO => None,
            n => Some(n),
        }
    }
    fn add_applied_dmg_for_tick_range(&self, start_tick: Count, end_tick: Count) -> PValue {
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
}
