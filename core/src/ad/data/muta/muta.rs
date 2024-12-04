use crate::{
    ad::AMutaAttrRange,
    defs::{EAttrId, EItemId},
    util::StMap,
};

/// Represents an adapted mutator (aka mutaplasmid in EVE).
///
/// A mutator controls how attributes of an item it is being applied to change.
pub struct AMuta {
    /// Mutator item type ID.
    pub id: EItemId,
    /// Describes which item you will get (value) by applying the mutator to another item (key).
    pub item_map: StMap<EItemId, EItemId>,
    /// Describes mutation ranges for attributes.
    pub attr_mods: StMap<EAttrId, AMutaAttrRange>,
}
impl AMuta {
    /// Make a new adapted mutator out of passed data.
    pub(crate) fn new(id: EItemId) -> Self {
        Self {
            id,
            item_map: StMap::new(),
            attr_mods: StMap::new(),
        }
    }
}
