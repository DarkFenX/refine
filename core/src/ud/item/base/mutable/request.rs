use crate::{
    ad::{AAttrId, AItemId},
    misc::UnitInterval,
};

pub(crate) struct UItemMutationRequest {
    pub(crate) mutator_id: AItemId,
    pub(crate) attrs: Vec<UAttrMutationRequest>,
}

pub(crate) struct UAttrMutationRequest {
    pub(crate) attr_id: AAttrId,
    pub(crate) value: Option<UnitInterval>,
}
