use crate::{
    dbg::{DebugResult, check_item_key},
    ud::{UData, item::misc::UAutocharges},
};

impl UAutocharges {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for autocharge_key in self.values() {
            // All autocharges are supposed to be loaded
            check_item_key(u_data, autocharge_key, true)?;
        }
        Ok(())
    }
}
