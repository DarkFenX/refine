use crate::{ad, defs::EAttrId};

pub struct SolSideEffectInfo {
    pub status: bool,
    pub chance_attr_id: EAttrId,
    pub strength: Option<SolSideEffectStr>,
}
impl SolSideEffectInfo {
    pub(in crate::sol) fn new(status: bool, chance_attr_id: EAttrId, strength: Option<SolSideEffectStr>) -> Self {
        Self {
            status,
            chance_attr_id,
            strength,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SolSideEffectStr {
    pub op: ad::AOp,
    pub attr_id: EAttrId,
}
impl SolSideEffectStr {
    pub(in crate::sol) fn new(op: ad::AOp, attr_id: EAttrId) -> Self {
        Self { op, attr_id }
    }
}
