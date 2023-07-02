use std::collections::HashMap;

use crate::{
    ad::AMutaAttrRange,
    defs::{EAttrId, EItemId, EMutaId},
    util::Named,
};

/// Represents an adapted mutaplasmid.
///
/// A mutaplasmid controls how attributes of an item it is being applied to change.
#[derive(Debug)]
pub struct AMuta {
    /// Mutaplasmid ID.
    pub id: EMutaId,
    /// Describes which item you will get (value) by applying the mutaplasmid to another item (key).
    pub item_map: HashMap<EItemId, EItemId>,
    /// Describes mutation ranges for attributes.
    pub attr_mods: HashMap<EAttrId, AMutaAttrRange>,
}
impl AMuta {
    /// Make a new adapted mutaplasmid out of passed data.
    pub(crate) fn new(id: EMutaId) -> Self {
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
