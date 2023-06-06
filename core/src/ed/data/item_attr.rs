use crate::{
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// An EVE item type-attribute relation.
#[derive(Debug)]
pub struct EItemAttr {
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers an attribute involved in the relation.
    pub attr_id: ReeInt,
    /// Value of the attribute.
    pub value: ReeFloat,
}
impl EItemAttr {
    /// Make a new item-attribute relation out of passed data.
    pub fn new(item_id: ReeInt, attr_id: ReeInt, value: ReeFloat) -> Self {
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
