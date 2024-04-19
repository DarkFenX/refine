use crate::{
    defs::{EAttrId, SsItemId},
    ss::svc::svce_calc::SsAttrVal,
    util::{Error, ErrorKind, Result, StMap},
};

pub(in crate::ss::svc::svce_calc) struct SsAttrValData {
    pub(super) data: StMap<SsItemId, StMap<EAttrId, SsAttrVal>>,
}
impl SsAttrValData {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_item_attrs(
        &self,
        item_id: &SsItemId,
    ) -> Result<&StMap<EAttrId, SsAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub(in crate::ss::svc::svce_calc) fn get_item_attrs_mut(
        &mut self,
        item_id: &SsItemId,
    ) -> Result<&mut StMap<EAttrId, SsAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn add_item(&mut self, item_id: SsItemId) {
        self.data.insert(item_id, StMap::new());
    }
    pub(in crate::ss::svc::svce_calc) fn remove_item(&mut self, item_id: &SsItemId) {
        self.data.remove(item_id);
    }
}
