use crate::{
    dbg::{DebugResult, check_item_id},
    ud::{UData, item::misc::UAutocharges},
};

impl UAutocharges {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for autocharge_key in self.values() {
            // All autocharges are supposed to be loaded
            check_item_id(u_data, autocharge_key, true)?;
        }
        Ok(())
    }
}
