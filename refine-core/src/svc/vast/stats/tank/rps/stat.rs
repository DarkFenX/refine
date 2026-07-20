use crate::num::PValue;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatRps {
    pub shield: StatRpsLayerRegen,
    pub armor: StatRpsLayer,
    pub hull: StatRpsLayer,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatRpsLayerRegen {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
    pub regen: PValue,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatRpsLayer {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
}
