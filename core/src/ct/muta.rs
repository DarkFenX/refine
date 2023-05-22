use std::collections::HashMap;

use crate::{
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// Represents a mutaplasmid.
///
/// A mutaplasmid controls how attributes of an item it is being applied to change.
#[derive(Debug, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub struct Muta {
    /// Mutaplasmid ID.
    pub id: ReeInt,
    /// Describes which item you will get (value) by applying the mutaplasmid to another item (key).
    pub item_map: HashMap<ReeInt, ReeInt>,
    /// Describes mutation ranges for attributes.
    pub attr_mods: HashMap<ReeInt, MutaAttrRange>,
}
impl Muta {
    /// Make a new mutaplasmid out of passed data.
    pub(crate) fn new(id: ReeInt) -> Self {
        Self {
            id,
            item_map: HashMap::new(),
            attr_mods: HashMap::new(),
        }
    }
}
impl Named for Muta {
    fn get_name() -> &'static str {
        "ct::Muta"
    }
}

/// Stores mutation range of specific attribute of specific mutaplasmid.
#[derive(Debug, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub struct MutaAttrRange {
    /// Lower boundary of the modification range.
    pub min_mult: ReeFloat,
    /// Upper boundary of the modification range.
    pub max_mult: ReeFloat,
}
impl MutaAttrRange {
    /// Make a new attribute mutation range.
    pub(crate) fn new(min_mult: ReeFloat, max_mult: ReeFloat) -> Self {
        Self { min_mult, max_mult }
    }
}
impl Named for MutaAttrRange {
    fn get_name() -> &'static str {
        "ct::MutaAttrRange"
    }
}
