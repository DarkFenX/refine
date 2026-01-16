use super::AttrValData;
use crate::{dbg::DebugResult, ud::UData};

impl AttrValData {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (item_uid, item_data) in self.data.iter() {
            item_uid.consistency_check(u_data, true)?;
            for attr_rid in item_data.keys() {
                attr_rid.consistency_check(u_data)?;
            }
        }
        Ok(())
    }
}
