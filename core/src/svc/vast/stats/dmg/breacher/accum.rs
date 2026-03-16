use std::collections::hash_map::Entry;

use super::{conv::cseq_to_ticks, ticks::AggrBreacherTicks};
use crate::{
    def::SERVER_TICK_HZ,
    misc::Breacher,
    num::{Count, PValue, UnitInterval},
    svc::{
        cycle::{CycleDataDur, CycleSeq},
        output::Output,
        vast::StatDmgEntryBreacher,
    },
    util::RMap,
};

const DAY_TICKS: Count = Count::from_u32(24 * 60 * 60 * SERVER_TICK_HZ as u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct BreacherAccumEntry {
    absolute_max: PValue,
    relative_max: UnitInterval,
    ticks: AggrBreacherTicks,
}

pub(in crate::svc::vast::stats::dmg) struct BreacherAccum {
    data: RMap<BreacherAccumEntry, Count>,
}
impl BreacherAccum {
    pub(in crate::svc::vast) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(in crate::svc::vast) fn add(&mut self, cseq: CycleSeq<CycleDataDur>, output: Output<Breacher>) {
        let ticks = match cseq_to_ticks(cseq, output) {
            Some(ticks) => ticks,
            None => return,
        };
        let accum_entry = BreacherAccumEntry {
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
}
