use crate::{
    ad::{AAttrId, AItemId, AMutaAttrRange},
    util::HMap,
};

/// Represents an adapted mutator (aka mutaplasmid in EVE).
///
/// A mutator controls how attributes of an item it is being applied to change.
pub struct AMuta {
    /// Mutator item type ID.
    pub id: AItemId,
    /// Describes which item you will get (value) by applying the mutator to another item (key).
    pub item_map: HMap<AItemId, AItemId>,
    /// Describes mutation ranges for attributes.
    pub attr_mods: HMap<AAttrId, AMutaAttrRange>,
}
impl AMuta {
    /// Make a new adapted mutator out of passed data.
    pub(crate) fn new(id: AItemId) -> Self {
        Self {
            id,
            item_map: HMap::new(),
            attr_mods: HMap::new(),
        }
    }
}
