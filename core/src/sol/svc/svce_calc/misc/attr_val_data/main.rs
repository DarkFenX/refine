use crate::{
    defs::{EAttrId, SolItemId},
    sol::svc::svce_calc::SolAttrVal,
    util::{Error, ErrorKind, Result, StMap},
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_calc) struct SolAttrValData {
    pub(super) data: StMap<SolItemId, StMap<EAttrId, SolAttrVal>>,
}
impl SolAttrValData {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_item_attrs(
        &self,
        item_id: &SolItemId,
    ) -> Result<&StMap<EAttrId, SolAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound(*item_id)))
    }
    pub(in crate::sol::svc::svce_calc) fn get_item_attrs_mut(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<&mut StMap<EAttrId, SolAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound(*item_id)))
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn add_item(&mut self, item_id: SolItemId) {
        self.data.insert(item_id, StMap::new());
    }
    pub(in crate::sol::svc::svce_calc) fn remove_item(&mut self, item_id: &SolItemId) {
        self.data.remove(item_id);
    }
}
