use crate::{
    ed::{EAttrId, EAttrVal, EItemId},
    util::Named,
};

/// An EVE item type-attribute relation.
pub struct EItemAttr {
    /// Refers an item type involved in the relation.
    pub item_id: EItemId,
    /// Refers an attribute involved in the relation.
    pub attr_id: EAttrId,
    /// Value of the attribute.
    pub value: EAttrVal,
}
impl Named for EItemAttr {
    fn get_name() -> &'static str {
        "EItemAttr"
    }
}
