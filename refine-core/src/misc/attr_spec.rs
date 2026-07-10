use crate::{
    dbg::DebugResult,
    rd::RAttrId,
    ud::{UData, UItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AttrSpec {
    pub(crate) item_uid: UItemId,
    pub(crate) attr_rid: RAttrId,
}
impl AttrSpec {
    pub(crate) fn new(item_uid: UItemId, attr_rid: RAttrId) -> Self {
        Self { item_uid, attr_rid }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AttrSpec {
    pub(crate) fn consistency_check(&self, u_data: &UData, check_item_load: bool) -> DebugResult {
        self.item_uid.consistency_check(u_data, check_item_load)?;
        self.attr_rid.consistency_check(u_data)?;
        Ok(())
    }
}
