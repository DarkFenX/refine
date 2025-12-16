use crate::ad::{ACustomAttrId, AEveAttrId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AAttrId {
    Eve(AEveAttrId),
    Custom(ACustomAttrId),
}
impl std::fmt::Display for AAttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}
