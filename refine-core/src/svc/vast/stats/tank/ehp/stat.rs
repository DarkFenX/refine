use crate::num::PValue;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatEhp {
    pub shield: Option<StatEhpLayer>,
    pub armor: Option<StatEhpLayer>,
    pub hull: Option<StatEhpLayer>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatEhpLayer {
    pub buffer: PValue,
    pub ancil_local: PValue,
    pub ancil_remote: PValue,
    pub mult: PValue,
}
