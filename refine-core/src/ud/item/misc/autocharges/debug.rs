use crate::{
    dbg::DebugResult,
    ud::{UData, item::misc::UAutocharges},
};

impl UAutocharges {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for autocharge_uid in self.values() {
            // All autocharges are supposed to be loaded
            autocharge_uid.consistency_check(u_data, true)?;
        }
        Ok(())
    }
}
