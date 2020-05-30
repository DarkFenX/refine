use std::collections::HashMap;

use crate::defines::{ReeFloat, ReeInt};

/// Represents a mutaplasmid.
///
/// A mutaplasmid controls how attributes of an item it is being applied to change.
#[derive(Debug)]
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
    pub fn new(id: ReeInt, item_map: HashMap<ReeInt, ReeInt>, attr_mods: HashMap<ReeInt, MutaAttrRange>) -> Muta {
        Muta {
            id,
            item_map,
            attr_mods,
        }
    }
}

/// Stores mutation range of specific attribute of specific mutaplasmid.
#[derive(Debug)]
pub struct MutaAttrRange {
    /// Lower boundary of the modification range.
    pub min_mult: ReeFloat,
    /// Upper boundary of the modification range.
    pub max_mult: ReeFloat,
}
impl MutaAttrRange {
    /// Make a new attribute mutation range.
    pub fn new(min_mult: ReeFloat, max_mult: ReeFloat) -> MutaAttrRange {
        MutaAttrRange { min_mult, max_mult }
    }
}
