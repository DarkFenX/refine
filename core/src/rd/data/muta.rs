use crate::{
    ad,
    rd::{RAttrKey, RItemKey},
    util::RMap,
};

pub(crate) struct RMuta {
    a_muta: ad::AMuta,
    item_map: RMap<RItemKey, RItemKey>,
    attr_mods: RMap<RAttrKey, ad::AMutaAttrRange>,
}
impl RMuta {
    pub(crate) fn new(a_muta: ad::AMuta) -> Self {
        Self {
            a_muta,
            item_map: RMap::new(),
            attr_mods: RMap::new(),
        }
    }
    fn fill_r_keys(&mut self) {
        // TODO: add actual contents which fill min/max keys
    }
    pub(crate) fn get_item_map(&self) -> &RMap<RItemKey, RItemKey> {
        &self.item_map
    }
    pub(crate) fn get_attr_mods(&self) -> &RMap<RAttrKey, ad::AMutaAttrRange> {
        &self.attr_mods
    }
}
