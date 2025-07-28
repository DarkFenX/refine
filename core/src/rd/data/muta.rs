use crate::{
    ad::{AAttrId, AItemId, AMuta, AMutaAttrRange},
    util::{GetId, Named, RMap},
};

// Represents a mutator (aka mutaplasmid in EVE).
//
// A mutator controls how attributes of an item it is being applied to change.
pub(crate) struct RMuta {
    a_muta: AMuta,
}
impl RMuta {
    pub(in crate::rd) fn new(a_muta: AMuta) -> Self {
        Self { a_muta }
    }
    pub(crate) fn get_item_map(&self) -> &RMap<AItemId, AItemId> {
        &self.a_muta.item_map
    }
    pub(crate) fn get_attr_mods(&self) -> &RMap<AAttrId, AMutaAttrRange> {
        &self.a_muta.attr_mods
    }
}
impl GetId<AItemId> for RMuta {
    fn get_id(&self) -> AItemId {
        self.a_muta.id
    }
}
impl Named for RMuta {
    fn get_name() -> &'static str {
        "RMuta"
    }
}
