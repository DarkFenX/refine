use crate::num::PValue;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatOutReps {
    pub shield: PValue,
    pub armor: PValue,
    pub hull: PValue,
}
