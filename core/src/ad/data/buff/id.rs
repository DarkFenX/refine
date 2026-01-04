use crate::{def::DefId, ed::EBuffId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ABuffId {
    Eve(AEveBuffId),
    Custom(ACustomBuffId),
}
impl const From<EBuffId> for ABuffId {
    fn from(buff_eid: EBuffId) -> Self {
        Self::Eve(AEveBuffId(buff_eid.into_inner()))
    }
}
impl std::fmt::Display for ABuffId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AEveBuffId(DefId);
impl AEveBuffId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ACustomBuffId(DefId);
impl ACustomBuffId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
