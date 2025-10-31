#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatSensor {
    kind: HStatSensorKind,
    strength: rc::AttrVal,
}
impl From<rc::stats::StatSensor> for HStatSensor {
    fn from(core_stat: rc::stats::StatSensor) -> Self {
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
impl From<rc::stats::StatSensorKind> for HStatSensorKind {
    fn from(core_stat: rc::stats::StatSensorKind) -> Self {
        match core_stat {
            rc::stats::StatSensorKind::Radar => Self::Radar,
            rc::stats::StatSensorKind::Gravimetric => Self::Gravimetric,
            rc::stats::StatSensorKind::Magnetometric => Self::Magnetometric,
            rc::stats::StatSensorKind::Ladar => Self::Ladar,
        }
    }
}
