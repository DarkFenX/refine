use crate::{
    ad::{AAttrId, AItemId},
    util::UnitInterval,
};

pub(crate) struct ItemMutationRequest {
    pub(crate) mutator_id: AItemId,
    pub(crate) attrs: Vec<AttrMutationRequest>,
}

pub(crate) struct AttrMutationRequest {
    pub(crate) attr_id: AAttrId,
    pub(crate) value: Option<UnitInterval>,
}
