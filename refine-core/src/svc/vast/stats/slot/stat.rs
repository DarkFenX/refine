use crate::num::Count;

pub struct StatSlot {
    pub used: Count,
    pub total: Option<Count>,
}
