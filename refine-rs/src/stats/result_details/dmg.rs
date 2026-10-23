use crate::PValue;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatDmg {
    pub dps: StatDmgEntry,
    pub volley: StatDmgEntry,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatDmgEntry {
    pub em: PValue,
    pub thermal: PValue,
    pub kinetic: PValue,
    pub explosive: PValue,
    pub breacher: StatDmgEntryBreacher,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[derive(Copy, Clone)]
pub enum StatDmgEntryBreacher {
    Raw(StatDmgEntryBreacherRaw),
    Applied(PValue),
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatDmgEntryBreacherRaw {
    pub absolute_max: PValue,
    pub relative_max: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatDmg {
    pub(in crate::stats) fn from_core(core_stat: rc::stats::StatDmg) -> Self {
        let rc::stats::StatDmg { dps, volley } = core_stat;
        Self {
            dps: StatDmgEntry::from_core(dps),
            volley: StatDmgEntry::from_core(volley),
        }
    }
    pub(in crate::stats) fn from_core_applied(core_stat: rc::stats::StatDmgApplied) -> Self {
        let rc::stats::StatDmgApplied { dps, volley } = core_stat;
        Self {
            dps: StatDmgEntry::from_core_applied(dps),
            volley: StatDmgEntry::from_core_applied(volley),
        }
    }
}

impl StatDmgEntry {
    fn from_core(core_stat: rc::stats::StatDmgEntry) -> Self {
        let rc::stats::StatDmgEntry {
            em,
            thermal,
            kinetic,
            explosive,
            breacher,
        } = core_stat;
        Self {
            em,
            thermal,
            kinetic,
            explosive,
            breacher: StatDmgEntryBreacher::from_core(breacher),
        }
    }
    fn from_core_applied(core_stat: rc::stats::StatDmgEntryApplied) -> Self {
        let rc::stats::StatDmgEntryApplied {
            em,
            thermal,
            kinetic,
            explosive,
            breacher,
        } = core_stat;
        Self {
            em,
            thermal,
            kinetic,
            explosive,
            breacher: StatDmgEntryBreacher::from_core_applied(breacher),
        }
    }
}

impl StatDmgEntryBreacher {
    fn from_core(core_stat: rc::stats::StatDmgEntryBreacher) -> Self {
        let rc::stats::StatDmgEntryBreacher {
            absolute_max,
            relative_max,
        } = core_stat;
        Self::Raw(StatDmgEntryBreacherRaw {
            absolute_max,
            relative_max,
        })
    }
    fn from_core_applied(core_value: PValue) -> Self {
        Self::Applied(core_value)
    }
}
