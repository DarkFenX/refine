use crate::{ad::AAttr, ed::EAttrId};

impl AAttr {
    pub(in crate::ad::generator::rels) fn iter_attr_eids(&self) -> impl Iterator<Item = EAttrId> {
        let id = self.id.dc_eve().into_iter();
        let min = self.min_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let max = self.max_attr_id.and_then(|v| v.dc_eve()).into_iter();
        id.chain(min).chain(max)
    }
}
