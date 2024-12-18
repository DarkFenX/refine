use crate::defs::{AttrVal, EAttrId, EItemId, MutaRoll};

/// Specifies how item should be mutated.
pub struct SolItemAddMutation {
    /// Mutator type ID.
    pub mutator_id: EItemId,
    /// Attribute mutation list.
    pub attrs: Vec<SolItemAddAttrMutation>,
}
impl SolItemAddMutation {
    pub fn new(mutator_id: EItemId) -> Self {
        Self {
            mutator_id,
            attrs: Vec::new(),
        }
    }
    pub fn new_with_attrs(mutator_id: EItemId, attrs: Vec<SolItemAddAttrMutation>) -> Self {
        Self { mutator_id, attrs }
    }
}

/// Specifies single attribute mutation.
pub struct SolItemAddAttrMutation {
    /// ID of attribute to mutate.
    pub attr_id: EAttrId,
    /// Mutation value.
    pub value: SolItemAttrMutationValue,
}
impl SolItemAddAttrMutation {
    pub fn new(attr_id: EAttrId, value: SolItemAttrMutationValue) -> Self {
        Self { attr_id, value }
    }
}

/// Specifies single attribute mutation.
pub struct SolItemChangeAttrMutation {
    /// ID of attribute to mutate.
    pub attr_id: EAttrId,
    /// Mutation value, None to remove.
    pub value: Option<SolItemAttrMutationValue>,
}
impl SolItemChangeAttrMutation {
    pub fn new(attr_id: EAttrId, value: Option<SolItemAttrMutationValue>) -> Self {
        Self { attr_id, value }
    }
}

/// Specifies value of a single attribute mutation.
pub enum SolItemAttrMutationValue {
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
