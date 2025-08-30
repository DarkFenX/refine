use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct Sensor {
    pub kind: SensorKind,
    pub strength: AttrVal,
}

#[derive(Copy, Clone)]
pub enum SensorKind {
    Radar,
    Gravimetric,
    Magnetometric,
    Ladar,
}
