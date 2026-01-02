use super::AttrValData;
use crate::{
    dbg::{DebugResult, check_attr_id, check_item_id},
    ud::UData,
};

impl AttrValData {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (item_key, item_data) in self.data.iter() {
            check_item_id(u_data, *item_key, true)?;
            for &attr_key in item_data.keys() {
                check_attr_id(u_data, attr_key)?;
            }
        }
        Ok(())
    }
}
