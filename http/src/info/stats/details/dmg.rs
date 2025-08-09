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
            breacher: core_stat.breacher.map(|v| v.into()),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatDmgBreacher {
    absolute_max: rc::AttrVal,
    relative_max: rc::AttrVal,
}
impl From<rc::stats::StatDmgBreacher> for HStatDmgBreacher {
    fn from(core_stat: rc::stats::StatDmgBreacher) -> Self {
        Self {
            absolute_max: core_stat.absolute_max,
            relative_max: core_stat.relative_max,
        }
    }
}
