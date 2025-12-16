use crate::{
    ad::{ACustomAttrId, AEveAttrId},
    ed::EAttrId,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AAttrId {
    Eve(AEveAttrId),
    Custom(ACustomAttrId),
}
impl AAttrId {
    pub(crate) fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            AAttrId::Eve(eve_attr_id) => Some(*eve_attr_id),
            AAttrId::Custom(_) => None,
        }
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
