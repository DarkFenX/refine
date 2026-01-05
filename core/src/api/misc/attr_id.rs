use crate::{
    ad::AAttrId,
    def::{CustomAttrId, EveAttrId},
};

/// ID of an attribute.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AttrId {
    /// ID of an EVE attribute.
    Eve(EveAttrId),
    /// ID of an attribute created by the library.
    Custom(CustomAttrId),
}
impl std::fmt::Display for AttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}
impl From<AAttrId> for AttrId {
    fn from(a_attr_id: AAttrId) -> Self {
        match a_attr_id {
            AAttrId::Eve(id) => Self::Eve(id),
            AAttrId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&AAttrId> for AttrId {
    fn from(a_attr_id: &AAttrId) -> Self {
        match a_attr_id {
            AAttrId::Eve(id) => Self::Eve(*id),
            AAttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<AttrId> for AAttrId {
    fn from(attr_id: AttrId) -> Self {
        match attr_id {
            AttrId::Eve(id) => Self::Eve(id),
            AttrId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&AttrId> for AAttrId {
    fn from(attr_id: &AttrId) -> Self {
        match attr_id {
            AttrId::Eve(id) => Self::Eve(*id),
            AttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
