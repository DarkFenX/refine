use std::collections::HashMap;

use crate::{
    defs::{EAttrId, SsItemId},
    util::{Error, ErrorKind, Result},
};

use super::SsAttrVal;

pub(in crate::ss::svc::svce_calc) struct AttrValData {
    data: HashMap<SsItemId, HashMap<EAttrId, SsAttrVal>>,
}
impl AttrValData {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self { data: HashMap::new() }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_item_attrs(
        &self,
        item_id: &SsItemId,
    ) -> Result<&HashMap<EAttrId, SsAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub(in crate::ss::svc::svce_calc) fn get_item_attrs_mut(
        &mut self,
        item_id: &SsItemId,
    ) -> Result<&mut HashMap<EAttrId, SsAttrVal>> {
        // All items known to calculator should be added to the map, so consider absence an error
        self.data
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn add_item(&mut self, item_id: SsItemId) {
        self.data.insert(item_id, HashMap::new());
    }
    pub(in crate::ss::svc::svce_calc) fn remove_item(&mut self, item_id: &SsItemId) {
        self.data.remove(item_id);
    }
}
