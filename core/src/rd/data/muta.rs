use crate::{
    ad,
    util::{Named, RMap},
};

// Represents a mutator (aka mutaplasmid in EVE).
//
// A mutator controls how attributes of an item it is being applied to change.
pub(crate) struct RMuta {
    a_muta: ad::AMuta,
}
impl RMuta {
    pub(crate) fn new(a_muta: ad::AMuta) -> Self {
        Self { a_muta }
    }
    // Describes which item you will get (value) by applying the mutator to another item (key).
    pub(crate) fn get_item_map(&self) -> &RMap<ad::AItemId, ad::AItemId> {
        &self.a_muta.item_map
    }
    // Describes mutation ranges for attributes.
    pub(crate) fn get_attr_mods(&self) -> &RMap<ad::AAttrId, ad::AMutaAttrRange> {
        &self.a_muta.attr_mods
    }
}
impl Named for RMuta {
    fn get_name() -> &'static str {
        "RMuta"
    }
}
