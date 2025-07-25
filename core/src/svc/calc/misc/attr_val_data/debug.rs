use super::AttrValData;
use crate::{
    dbg::{DebugResult, check_item_key},
    ud::UData,
};

impl AttrValData {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for item_key in self.data.keys() {
            check_item_key(u_data, *item_key, true)?;
        }
        Ok(())
    }
}
