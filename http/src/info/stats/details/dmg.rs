use serde::Serialize;
use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatDmg {
    em: f64,
    thermal: f64,
    kinetic: f64,
    explosive: f64,
    breacher: Option<HStatDmgBreacher>,
}
impl From<rc::stats::StatDmg> for HStatDmg {
    fn from(core_stat: rc::stats::StatDmg) -> Self {
        Self {
            em: core_stat.em.into_f64(),
            thermal: core_stat.thermal.into_f64(),
            kinetic: core_stat.kinetic.into_f64(),
            explosive: core_stat.explosive.into_f64(),
            breacher: core_stat.breacher.map(Into::into),
        }
    }
}
impl From<rc::stats::StatDmgApplied> for HStatDmg {
    fn from(core_stat: rc::stats::StatDmgApplied) -> Self {
        Self {
            em: core_stat.em.into_f64(),
            thermal: core_stat.thermal.into_f64(),
            kinetic: core_stat.kinetic.into_f64(),
            explosive: core_stat.explosive.into_f64(),
            breacher: core_stat.breacher.map(Into::into),
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
enum HStatDmgBreacher {
    Raw(HStatDmgBreacherRaw),
    Applied(f64),
}
impl From<rc::stats::StatDmgBreacher> for HStatDmgBreacher {
    fn from(core_stat: rc::stats::StatDmgBreacher) -> Self {
        Self::Raw(HStatDmgBreacherRaw {
            absolute_max: core_stat.absolute_max.into_f64(),
            relative_max: core_stat.relative_max.into_f64(),
        })
    }
}
impl From<rc::PValue> for HStatDmgBreacher {
    fn from(core_value: rc::PValue) -> Self {
        Self::Applied(core_value.into_f64())
    }
}

#[derive(Serialize_tuple)]
struct HStatDmgBreacherRaw {
    absolute_max: f64,
    relative_max: f64,
}
