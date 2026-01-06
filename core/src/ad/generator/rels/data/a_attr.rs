use crate::{
    ad::{AAttr, AAttrId},
    ed::EAttrId,
};

impl AAttr {
    pub(in crate::ad::generator::rels) fn iter_attr_eids(&self) -> impl Iterator<Item = EAttrId> {
        let id = self.id.dc_eve().into_iter();
        let min = self.min_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let max = self.max_attr_id.and_then(|v| v.dc_eve()).into_iter();
        id.chain(min).chain(max)
    }
}

impl AAttrId {
    pub(super) fn dc_eve(&self) -> Option<EAttrId> {
        match self {
            AAttrId::Eve(eve_attr_aid) => Some(EAttrId::from_i32(eve_attr_aid.into_i32())),
            AAttrId::Custom(_) => None,
        }
    }
}
