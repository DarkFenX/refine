use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct StatSensors {
    pub kind: StatSensorsKind,
    pub strength: AttrVal,
}

#[derive(Copy, Clone)]
pub enum StatSensorsKind {
    Radar,
    Magnetometric,
    Gravimetric,
    Ladar,
}
