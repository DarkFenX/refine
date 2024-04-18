use crate::{
    ad::AMutaAttrRange,
    defs::{EAttrId, EItemId, EMutaId},
    util::StMap,
};

/// Represents an adapted mutaplasmid.
///
/// A mutaplasmid controls how attributes of an item it is being applied to change.
pub struct AMuta {
    /// Mutaplasmid ID.
    pub id: EMutaId,
    /// Describes which item you will get (value) by applying the mutaplasmid to another item (key).
    pub item_map: StMap<EItemId, EItemId>,
    /// Describes mutation ranges for attributes.
    pub attr_mods: StMap<EAttrId, AMutaAttrRange>,
}
impl AMuta {
    /// Make a new adapted mutaplasmid out of passed data.
    pub(crate) fn new(id: EMutaId) -> Self {
        Self {
            id,
            item_map: StMap::new(),
            attr_mods: StMap::new(),
        }
    }
}
