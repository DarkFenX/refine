use std::collections::HashMap;

use crate::{
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// Represents an adapted mutaplasmid.
///
/// A mutaplasmid controls how attributes of an item it is being applied to change.
#[derive(Debug)]
pub struct AMuta {
    /// Mutaplasmid ID.
    pub id: ReeInt,
    /// Describes which item you will get (value) by applying the mutaplasmid to another item (key).
    pub item_map: HashMap<ReeInt, ReeInt>,
    /// Describes mutation ranges for attributes.
    pub attr_mods: HashMap<ReeInt, AMutaAttrRange>,
}
impl AMuta {
    /// Make a new adapted mutaplasmid out of passed data.
    pub(crate) fn new(id: ReeInt) -> Self {
        Self {
            id,
            item_map: HashMap::new(),
            attr_mods: HashMap::new(),
        }
    }
}
impl Named for AMuta {
    fn get_name() -> &'static str {
        "AMuta"
    }
}

/// Stores mutation range of specific attribute of specific mutaplasmid.
#[derive(Debug)]
pub struct AMutaAttrRange {
    /// Lower boundary of the modification range.
    pub min_mult: ReeFloat,
    /// Upper boundary of the modification range.
    pub max_mult: ReeFloat,
}
impl AMutaAttrRange {
    /// Make a new attribute mutation range.
    pub(crate) fn new(min_mult: ReeFloat, max_mult: ReeFloat) -> Self {
        Self { min_mult, max_mult }
    }
}
impl Named for AMutaAttrRange {
    fn get_name() -> &'static str {
        "AMutaAttrRange"
    }
}
