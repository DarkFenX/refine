use crate::{
    ad,
    util::{Named, RMap},
};

pub(crate) struct RMuta {
    a_muta: ad::AMuta,
}
impl RMuta {
    pub(crate) fn new(a_muta: ad::AMuta) -> Self {
        Self { a_muta }
    }
    pub(crate) fn get_item_map(&self) -> &RMap<ad::AItemId, ad::AItemId> {
        &self.a_muta.item_map
    }
    pub(crate) fn get_attr_mods(&self) -> &RMap<ad::AAttrId, ad::AMutaAttrRange> {
        &self.a_muta.attr_mods
    }
}
impl Named for RMuta {
    fn get_name() -> &'static str {
        "RMuta"
    }
}
