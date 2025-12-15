use crate::ad::{ACustomBuffId, AEveBuffId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ABuffId {
    Eve(AEveBuffId),
    Custom(ACustomBuffId),
}
impl std::fmt::Display for ABuffId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}
