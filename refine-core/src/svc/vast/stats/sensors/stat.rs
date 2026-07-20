use crate::num::PValue;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatSensors {
    pub kind: StatSensorsKind,
    pub strength: PValue,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum StatSensorsKind {
    Radar,
    Magnetometric,
    Gravimetric,
    Ladar,
}
