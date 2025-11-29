#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatSensors {
    kind: HStatSensorKind,
    strength: rc::AttrVal,
}
impl From<rc::stats::StatSensors> for HStatSensors {
    fn from(core_stat: rc::stats::StatSensors) -> Self {
        Self {
            kind: core_stat.kind.into(),
            strength: core_stat.strength,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum HStatSensorKind {
    Radar,
    Gravimetric,
    Magnetometric,
    Ladar,
}
impl From<rc::stats::StatSensorsKind> for HStatSensorKind {
    fn from(core_stat: rc::stats::StatSensorsKind) -> Self {
        match core_stat {
            rc::stats::StatSensorsKind::Radar => Self::Radar,
            rc::stats::StatSensorsKind::Gravimetric => Self::Gravimetric,
            rc::stats::StatSensorsKind::Magnetometric => Self::Magnetometric,
            rc::stats::StatSensorsKind::Ladar => Self::Ladar,
        }
    }
}
