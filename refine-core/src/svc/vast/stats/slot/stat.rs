use crate::num::Count;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct StatSlot {
    pub used: Count,
    pub total: Option<Count>,
}
