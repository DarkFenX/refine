use crate::sol::{AttrId, OpInfo};

pub struct SideEffectInfo {
    pub chance_attr_id: AttrId,
    pub status: bool,
    pub strength: Option<SideEffectStr>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SideEffectStr {
    pub op: OpInfo,
    pub attr_id: AttrId,
}
