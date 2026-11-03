use crate::{
    UnitInterval,
    ad::{AAttrId, AItemId},
};

pub(crate) struct UItemMutationRequest {
    pub(crate) mutator_type_aid: AItemId,
    pub(crate) attrs: Vec<UAttrMutationRequest>,
}

#[derive(Copy, Clone)]
pub(crate) struct UAttrMutationRequest {
    pub(crate) attr_aid: AAttrId,
    pub(crate) roll: Option<UnitInterval>,
}
