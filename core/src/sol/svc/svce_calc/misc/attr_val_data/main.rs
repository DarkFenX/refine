use crate::{
    defs::{EAttrId, SolItemId},
    err::basic::ItemLoadedError,
    sol::svc::svce_calc::SolAttrVal,
    util::StMap,
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
    ) -> Result<&StMap<EAttrId, SolAttrVal>, ItemLoadedError> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data.get(item_id).ok_or_else(|| ItemLoadedError::new(*item_id))
    }
    pub(in crate::sol::svc::svce_calc) fn get_item_attrs_mut(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<&mut StMap<EAttrId, SolAttrVal>, ItemLoadedError> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data.get_mut(item_id).ok_or_else(|| ItemLoadedError::new(*item_id))
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn item_loaded(&mut self, item_id: SolItemId) {
        self.data.insert(item_id, StMap::new());
    }
    pub(in crate::sol::svc::svce_calc) fn item_unloaded(&mut self, item_id: &SolItemId) {
        self.data.remove(item_id);
    }
}
