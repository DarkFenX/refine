use serde::Serialize;
use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatSensors {
    kind: HStatSensorKind,
    strength: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum HStatSensorKind {
    Radar,
    Gravimetric,
    Magnetometric,
    Ladar,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatSensors {
    pub(crate) fn from_core(core_stat: rc::stats::StatSensors) -> Self {
        Self {
            kind: HStatSensorKind::from_core(core_stat.kind),
            strength: core_stat.strength.into_f64(),
        }
    }
}

impl HStatSensorKind {
    fn from_core(core_stat: rc::stats::StatSensorsKind) -> Self {
        match core_stat {
            rc::stats::StatSensorsKind::Radar => Self::Radar,
            rc::stats::StatSensorsKind::Gravimetric => Self::Gravimetric,
            rc::stats::StatSensorsKind::Magnetometric => Self::Magnetometric,
            rc::stats::StatSensorsKind::Ladar => Self::Ladar,
        }
    }
}
