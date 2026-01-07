use crate::misc::PValue;

#[derive(Copy, Clone)]
pub struct StatSensors {
    pub kind: StatSensorsKind,
    pub strength: PValue,
}

#[derive(Copy, Clone)]
pub enum StatSensorsKind {
    Radar,
    Magnetometric,
    Gravimetric,
    Ladar,
}
