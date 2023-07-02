use crate::{
    defs::{AttrVal, EAttrId, EItemId},
    util::Named,
};

/// An EVE item type-attribute relation.
pub struct EItemAttr {
    /// Refers an item type involved in the relation.
    pub item_id: EItemId,
    /// Refers an attribute involved in the relation.
    pub attr_id: EAttrId,
    /// Value of the attribute.
    pub value: AttrVal,
}
impl EItemAttr {
    /// Make a new item-attribute relation out of passed data.
    pub fn new(item_id: EItemId, attr_id: EAttrId, value: AttrVal) -> Self {
        Self {
            item_id,
            attr_id,
            value,
        }
    }
}
impl Named for EItemAttr {
    fn get_name() -> &'static str {
        "EItemAttr"
    }
}
