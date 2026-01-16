use super::AttrValData;
use crate::{dbg::DebugResult, ud::UData};

impl AttrValData {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (item_uid, item_data) in self.data.iter() {
            item_uid.consistency_check(u_data, true)?;
            for (attr_rid, attr_entry) in item_data.iter() {
                attr_rid.consistency_check(u_data)?;
                if let Some(attr_val) = attr_entry.value {
                    attr_val.base.consistency_check()?;
                    attr_val.dogma.consistency_check()?;
                    attr_val.extra.consistency_check()?;
                }
            }
        }
        Ok(())
    }
}
