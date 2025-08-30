#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HSensor {
    kind: HSensorKind,
    strength: rc::AttrVal,
}
impl From<rc::stats::Sensor> for HSensor {
    fn from(core_stat: rc::stats::Sensor) -> Self {
        Self {
            kind: core_stat.kind.into(),
            strength: core_stat.strength,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum HSensorKind {
    Radar,
    Gravimetric,
    Magnetometric,
    Ladar,
}
impl From<rc::stats::SensorKind> for HSensorKind {
    fn from(core_stat: rc::stats::SensorKind) -> Self {
        match core_stat {
            rc::stats::SensorKind::Radar => Self::Radar,
            rc::stats::SensorKind::Gravimetric => Self::Gravimetric,
            rc::stats::SensorKind::Magnetometric => Self::Magnetometric,
            rc::stats::SensorKind::Ladar => Self::Ladar,
        }
    }
}
