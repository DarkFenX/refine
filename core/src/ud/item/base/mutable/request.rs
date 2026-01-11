use crate::{
    ad::{AAttrId, AItemId},
    num::UnitInterval,
};

pub(crate) struct UItemMutationRequest {
    pub(crate) mutator_type_aid: AItemId,
    pub(crate) attrs: Vec<UAttrMutationRequest>,
}

pub(crate) struct UAttrMutationRequest {
    pub(crate) attr_aid: AAttrId,
    pub(crate) roll: Option<UnitInterval>,
}
