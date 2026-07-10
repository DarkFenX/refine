use crate::num::PValue;

#[derive(Copy, Clone)]
pub struct StatRps {
    pub shield: StatRpsLayerRegen,
    pub armor: StatRpsLayer,
    pub hull: StatRpsLayer,
}

#[derive(Copy, Clone)]
pub struct StatRpsLayerRegen {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
    pub regen: PValue,
}

#[derive(Copy, Clone)]
pub struct StatRpsLayer {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
}
