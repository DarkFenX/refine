use crate::{
    ad::{AAttrId, AAttrVal},
    util::Named,
};

/// Represents an adapted dogma attribute.
///
/// An attribute carries just properties which govern how modified attribute values are calculated.
/// Values themselves are stored on various items as plain numbers.
pub struct AAttr {
    /// Attribute ID.
    pub id: AAttrId,
    /// Defines if modifications applied to the attribute's values are immune to stacking penalties
    /// or not.
    pub penalizable: bool,
    /// "High is good" defines if higher value of the attribute is considered good or not.
    pub hig: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub def_val: AAttrVal,
    /// Refers another attribute, whose value limits minimum value of this attribute.
    pub min_attr_id: Option<AAttrId>,
    /// Refers another attribute, whose value limits maximum value of this attribute.
    pub max_attr_id: Option<AAttrId>,
}
impl Named for AAttr {
    fn get_name() -> &'static str {
        "AAttr"
    }
}
