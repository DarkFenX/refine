use crate::{
    ad,
    util::{GetId, Named, RMap},
};

// Represents a mutator (aka mutaplasmid in EVE).
//
// A mutator controls how attributes of an item it is being applied to change.
pub(crate) struct RMuta {
    a_muta: ad::AMuta,
}
impl RMuta {
    pub(in crate::rd) fn new(a_muta: ad::AMuta) -> Self {
        Self { a_muta }
    }
    pub(crate) fn get_item_map(&self) -> &RMap<ad::AItemId, ad::AItemId> {
        &self.a_muta.item_map
    }
    pub(crate) fn get_attr_mods(&self) -> &RMap<ad::AAttrId, ad::AMutaAttrRange> {
        &self.a_muta.attr_mods
    }
}
impl GetId<ad::AItemId> for RMuta {
    fn get_id(&self) -> ad::AItemId {
        self.a_muta.id
    }
}
impl Named for RMuta {
    fn get_name() -> &'static str {
        "RMuta"
    }
}
