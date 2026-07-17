use crate::PValue;

pub struct StatDmg {
    pub dps: StatDmgEntry,
    pub volley: StatDmgEntry,
}

pub struct StatDmgEntry {
    pub em: PValue,
    pub thermal: PValue,
    pub kinetic: PValue,
    pub explosive: PValue,
    pub breacher: StatDmgEntryBreacher,
}

pub enum StatDmgEntryBreacher {
    Raw(StatDmgEntryBreacherRaw),
    Applied(PValue),
}

pub struct StatDmgEntryBreacherRaw {
    pub absolute_max: PValue,
    pub relative_max: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatDmg {
    pub(crate) fn from_core(core_stat: rc::stats::StatDmg) -> Self {
        Self {
            dps: StatDmgEntry::from_core(core_stat.dps),
            volley: StatDmgEntry::from_core(core_stat.volley),
        }
    }
    pub(crate) fn from_core_applied(core_stat: rc::stats::StatDmgApplied) -> Self {
        Self {
            dps: StatDmgEntry::from_core_applied(core_stat.dps),
            volley: StatDmgEntry::from_core_applied(core_stat.volley),
        }
    }
}

impl StatDmgEntry {
    fn from_core(core_stat: rc::stats::StatDmgEntry) -> Self {
        Self {
            em: core_stat.em,
            thermal: core_stat.thermal,
            kinetic: core_stat.kinetic,
            explosive: core_stat.explosive,
            breacher: StatDmgEntryBreacher::from_core(core_stat.breacher),
        }
    }
    fn from_core_applied(core_stat: rc::stats::StatDmgEntryApplied) -> Self {
        Self {
            em: core_stat.em,
            thermal: core_stat.thermal,
            kinetic: core_stat.kinetic,
            explosive: core_stat.explosive,
            breacher: StatDmgEntryBreacher::from_core_applied(core_stat.breacher),
        }
    }
}

impl StatDmgEntryBreacher {
    fn from_core(core_stat: rc::stats::StatDmgEntryBreacher) -> Self {
        Self::Raw(StatDmgEntryBreacherRaw {
            absolute_max: core_stat.absolute_max,
            relative_max: core_stat.relative_max,
        })
    }
    fn from_core_applied(core_value: PValue) -> Self {
        Self::Applied(core_value)
    }
}
