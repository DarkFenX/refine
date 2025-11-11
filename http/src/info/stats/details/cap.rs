#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HStatCapSim {
    Stable(rc::AttrVal),
    Time(rc::AttrVal),
}
impl From<rc::stats::StatCapSim> for HStatCapSim {
    fn from(core_stat: rc::stats::StatCapSim) -> Self {
        match core_stat {
            rc::stats::StatCapSim::Stable(stability) => Self::Stable(stability.get_inner()),
            rc::stats::StatCapSim::Time(time) => Self::Time(time),
        }
    }
}
