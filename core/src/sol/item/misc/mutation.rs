use crate::{
    defs::{AttrVal, EAttrId, EItemId, MutaRoll},
    util::StMap,
};

/// Specifies how item should be mutated.
pub struct SolItemMutation {
    /// Mutator type ID.
    pub mutator_id: EItemId,
    /// Attribute mutation map.
    pub attrs: StMap<EAttrId, SolItemAttrMutation>,
}
impl SolItemMutation {
    pub fn new(mutator_id: EItemId) -> Self {
        Self {
            mutator_id,
            attrs: StMap::new(),
        }
    }
    pub fn new_with_attrs(mutator_id: EItemId, attrs: StMap<EAttrId, SolItemAttrMutation>) -> Self {
        Self { mutator_id, attrs }
    }
}

/// Specifies mutation of a single attribute.
pub enum SolItemAttrMutation {
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
