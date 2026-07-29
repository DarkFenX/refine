use crate::Count;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatSlot {
    pub used: Count,
    pub total: Option<Count>,
}
