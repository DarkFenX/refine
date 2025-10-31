use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct StatSensor {
    pub kind: StatSensorKind,
    pub strength: AttrVal,
}

#[derive(Copy, Clone)]
pub enum StatSensorKind {
    Radar,
    Magnetometric,
    Gravimetric,
    Ladar,
}
