use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HStatCapSim {
    Stable(f64),
    Time(f64),
}
impl HStatCapSim {
    pub(crate) fn from_core(core_stat: rc::stats::StatCapSim) -> Self {
        match core_stat {
            rc::stats::StatCapSim::Stable(stability) => Self::Stable(stability.into_f64()),
            rc::stats::StatCapSim::Time(time) => Self::Time(time.into()),
        }
    }
}
