use crate::{ad, util::UnitInterval};

pub(crate) struct ItemMutationRequest {
    pub(crate) mutator_id: ad::AItemId,
    pub(crate) attrs: Vec<AttrMutationRequest>,
}

pub(crate) struct AttrMutationRequest {
    pub(crate) attr_id: ad::AAttrId,
    pub(crate) value: Option<UnitInterval>,
}
