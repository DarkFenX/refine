use crate::{
    ad,
    sol::{AttrVal, ItemTypeId, UnitInterval},
};

pub(in crate::sol) struct ItemAddMutation {
    pub(in crate::sol) mutator_id: ItemTypeId,
    pub(in crate::sol) attrs: Vec<ItemAddAttrMutation>,
}
impl ItemAddMutation {
    pub(in crate::sol) fn new(mutator_id: ItemTypeId) -> Self {
        Self {
            mutator_id,
            attrs: Vec::new(),
        }
    }
    pub(in crate::sol) fn new_with_attrs(mutator_id: ItemTypeId, attrs: Vec<ItemAddAttrMutation>) -> Self {
        Self { mutator_id, attrs }
    }
}

pub(in crate::sol) struct ItemAddAttrMutation {
    pub(in crate::sol) a_attr_id: ad::AAttrId,
    pub(in crate::sol) value: ItemAttrMutationValue,
}
impl ItemAddAttrMutation {
    pub(in crate::sol) fn new(a_attr_id: ad::AAttrId, value: ItemAttrMutationValue) -> Self {
        Self { a_attr_id, value }
    }
}

pub(in crate::sol) struct ItemChangeAttrMutation {
    pub(in crate::sol) a_attr_id: ad::AAttrId,
    pub(in crate::sol) value: Option<ItemAttrMutationValue>,
}
impl ItemChangeAttrMutation {
    pub(in crate::sol) fn new(a_attr_id: ad::AAttrId, value: Option<ItemAttrMutationValue>) -> Self {
        Self { a_attr_id, value }
    }
}

pub(in crate::sol) enum ItemAttrMutationValue {
    Roll(UnitInterval),
    Absolute(AttrVal),
}
