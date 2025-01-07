use crate::{ad, defs::EAttrId};

pub struct SolSideEffectInfo {
    pub chance_attr_id: EAttrId,
    pub status: bool,
    pub strength: Option<SolSideEffectStr>,
}
impl SolSideEffectInfo {
    pub(in crate::sol) fn new(chance_attr_id: EAttrId, status: bool, strength: Option<SolSideEffectStr>) -> Self {
        Self {
            chance_attr_id,
            status,
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
