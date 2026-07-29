use crate::PValue;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatHp {
    pub shield: StatHpLayer,
    pub armor: StatHpLayer,
    pub hull: StatHpLayer,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatHpLayer {
    pub buffer: PValue,
    pub ancil_local: PValue,
    pub ancil_remote: PValue,
}
