use crate::PValue;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatErps {
    pub shield: Option<StatErpsLayerRegen>,
    pub armor: Option<StatErpsLayer>,
    pub hull: Option<StatErpsLayer>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatErpsLayerRegen {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
    pub regen: PValue,
    pub mult: PValue,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatErpsLayer {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
    pub mult: PValue,
}
