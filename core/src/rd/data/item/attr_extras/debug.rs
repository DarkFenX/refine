use crate::{
    dbg::{DebugResult, check_attr_rid},
    rd::RItemAXt,
    ud::UData,
};

impl RItemAXt {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attr_rid) = self.remote_resist_attr_rid {
            check_attr_rid(u_data, attr_rid)?;
        }
        Ok(())
    }
}
