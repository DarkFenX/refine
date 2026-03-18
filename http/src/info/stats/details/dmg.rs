use serde::Serialize;
use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatDmgEntry {
    em: f64,
    thermal: f64,
    kinetic: f64,
    explosive: f64,
    breacher: HStatDmgEntryBreacher,
}

#[derive(Serialize)]
#[serde(untagged)]
enum HStatDmgEntryBreacher {
    Raw(HStatDmgBreacherRaw),
    Applied(f64),
}

#[derive(Serialize_tuple)]
struct HStatDmgBreacherRaw {
    absolute_max: f64,
    relative_max: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatDmgEntry {
    pub(crate) fn from_core(core_stat: rc::stats::StatDmgEntry) -> Self {
        Self {
            em: core_stat.em.into_f64(),
            thermal: core_stat.thermal.into_f64(),
            kinetic: core_stat.kinetic.into_f64(),
            explosive: core_stat.explosive.into_f64(),
            breacher: HStatDmgEntryBreacher::from_core(core_stat.breacher),
        }
    }
    pub(crate) fn from_core_applied(core_stat: rc::stats::StatDmgEntryApplied) -> Self {
        Self {
            em: core_stat.em.into_f64(),
            thermal: core_stat.thermal.into_f64(),
            kinetic: core_stat.kinetic.into_f64(),
            explosive: core_stat.explosive.into_f64(),
            breacher: HStatDmgEntryBreacher::from_core_applied(core_stat.breacher),
        }
    }
}

impl HStatDmgEntryBreacher {
    fn from_core(core_stat: rc::stats::StatDmgEntryBreacher) -> Self {
        Self::Raw(HStatDmgBreacherRaw {
            absolute_max: core_stat.absolute_max.into_f64(),
            relative_max: core_stat.relative_max.into_f64(),
        })
    }
    fn from_core_applied(core_value: rc::PValue) -> Self {
        Self::Applied(core_value.into_f64())
    }
}
