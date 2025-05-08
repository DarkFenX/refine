use crate::{ad, sol::UnitInterval};

pub(in crate::sol) struct ItemMutationRequest {
    pub(in crate::sol) mutator_id: ad::AItemId,
    pub(in crate::sol) attrs: Vec<AttrMutationRequest>,
}

pub(in crate::sol) struct AttrMutationRequest {
    pub(in crate::sol) a_attr_id: ad::AAttrId,
    pub(in crate::sol) value: Option<UnitInterval>,
}
