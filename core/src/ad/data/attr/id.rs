use crate::{def::DefId, ed::EAttrId, util::f64_to_i32};

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
pub struct AEveAttrId(DefId);
impl AEveAttrId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub(crate) fn new_f64(id: f64) -> Self {
        Self(f64_to_i32(id))
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ACustomAttrId(DefId);
impl ACustomAttrId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
