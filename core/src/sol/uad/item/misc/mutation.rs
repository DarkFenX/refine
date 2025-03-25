use crate::sol::{AttrId, AttrVal, ItemTypeId, MutaRoll};

/// Specifies how item should be mutated.
pub struct ItemAddMutation {
    /// Mutator type ID.
    pub mutator_id: ItemTypeId,
    /// Attribute mutation list.
    pub attrs: Vec<ItemAddAttrMutation>,
}
impl ItemAddMutation {
    pub fn new(mutator_id: ItemTypeId) -> Self {
        Self {
            mutator_id,
            attrs: Vec::new(),
        }
    }
    pub fn new_with_attrs(mutator_id: ItemTypeId, attrs: Vec<ItemAddAttrMutation>) -> Self {
        Self { mutator_id, attrs }
    }
}

/// Specifies single attribute mutation.
pub struct ItemAddAttrMutation {
    /// ID of attribute to mutate.
    pub attr_id: AttrId,
    /// Mutation value.
    pub value: ItemAttrMutationValue,
}
impl ItemAddAttrMutation {
    pub fn new(attr_id: AttrId, value: ItemAttrMutationValue) -> Self {
        Self { attr_id, value }
    }
}

/// Specifies single attribute mutation.
pub struct ItemChangeAttrMutation {
    /// ID of attribute to mutate.
    pub attr_id: AttrId,
    /// Mutation value, None to remove.
    pub value: Option<ItemAttrMutationValue>,
}
impl ItemChangeAttrMutation {
    pub fn new(attr_id: AttrId, value: Option<ItemAttrMutationValue>) -> Self {
        Self { attr_id, value }
    }
}

/// Specifies value of a single attribute mutation.
pub enum ItemAttrMutationValue {
    /// Roll quality as a value on range \[0, 1\].
    Roll(MutaRoll),
    /// Absolute value of the attribute.
    ///
    /// Note that internally range value is used. To correctly interpret absolute value, current
    /// data source needs to have specified mutator, the mutator needs to have mutation for this
    /// attribute, and mutated item has to have base value for this attribute. Mutation gets
    /// discarded if any of those condition fail.
    Absolute(AttrVal),
}
