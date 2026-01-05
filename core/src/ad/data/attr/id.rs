use crate::{ed::EAttrId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AAttrId {
    Eve(AEveAttrId),
    Custom(ACustomAttrId),
}
impl const From<EAttrId> for AAttrId {
    fn from(attr_eid: EAttrId) -> Self {
        Self::Eve(AEveAttrId(attr_eid.into_inner()))
    }
}
impl std::fmt::Display for AAttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AEveAttrId(i32);
impl AEveAttrId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub(crate) fn new_f64_rounded(id: f64) -> Self {
        Self(round_f64_to_i32(id))
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ACustomAttrId(i32);
impl ACustomAttrId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}
