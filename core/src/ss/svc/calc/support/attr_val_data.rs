use std::collections::HashMap;

use crate::{
    defs::{AttrId, SsItemId},
    util::{Error, ErrorKind, Result},
};

use super::SsAttrVal;

pub(in crate::ss::svc::calc) struct AttrValData {
    data: HashMap<SsItemId, HashMap<AttrId, SsAttrVal>>,
}
impl AttrValData {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self { data: HashMap::new() }
    }
    // Getters
    pub(in crate::ss::svc::calc) fn get_item_attrs(&self, item_id: &SsItemId) -> Result<&HashMap<AttrId, SsAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub(in crate::ss::svc::calc) fn get_item_attrs_mut(
        &mut self,
        item_id: &SsItemId,
    ) -> Result<&mut HashMap<AttrId, SsAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    // Maintenance
    pub(in crate::ss::svc::calc) fn add_item(&mut self, item_id: SsItemId) {
        self.data.insert(item_id, HashMap::new());
    }
    pub(in crate::ss::svc::calc) fn remove_item(&mut self, item_id: &SsItemId) {
        self.data.remove(item_id);
    }
}
