use std::collections::hash_map::Entry;

use super::{conv::cseq_to_ticks, ticks::AggrBreacherTicks};
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
}
