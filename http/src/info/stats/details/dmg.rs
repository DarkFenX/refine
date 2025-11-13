#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatDmg {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
    breacher: Option<HStatDmgBreacher>,
}
impl From<rc::stats::StatDmg> for HStatDmg {
    fn from(core_stat: rc::stats::StatDmg) -> Self {
        Self {
            em: core_stat.em,
            thermal: core_stat.thermal,
            kinetic: core_stat.kinetic,
            explosive: core_stat.explosive,
            breacher: core_stat.breacher.map(Into::into),
        }
    }
}
impl From<rc::stats::StatDmgApplied> for HStatDmg {
    fn from(core_stat: rc::stats::StatDmgApplied) -> Self {
        Self {
            em: core_stat.em,
            thermal: core_stat.thermal,
            kinetic: core_stat.kinetic,
            explosive: core_stat.explosive,
            breacher: core_stat.breacher.map(Into::into),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
enum HStatDmgBreacher {
    Raw(HStatDmgBreacherRaw),
    Applied(rc::AttrVal),
}
impl From<rc::stats::StatDmgBreacher> for HStatDmgBreacher {
    fn from(core_stat: rc::stats::StatDmgBreacher) -> Self {
        Self::Raw(HStatDmgBreacherRaw {
            absolute_max: core_stat.absolute_max,
            relative_max: core_stat.relative_max,
        })
    }
}
impl From<rc::AttrVal> for HStatDmgBreacher {
    fn from(core_value: rc::AttrVal) -> Self {
        Self::Applied(core_value)
    }
}

#[derive(serde_tuple::Serialize_tuple)]
struct HStatDmgBreacherRaw {
    absolute_max: rc::AttrVal,
    relative_max: rc::AttrVal,
}
