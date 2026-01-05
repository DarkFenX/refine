use crate::{
    dbg::{DebugResult, check_item_uid},
    ud::{UData, item::misc::UAutocharges},
};

impl UAutocharges {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for autocharge_uid in self.values() {
            // All autocharges are supposed to be loaded
            check_item_uid(u_data, autocharge_uid, true)?;
        }
        Ok(())
    }
}
