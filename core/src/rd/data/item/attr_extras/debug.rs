use crate::{dbg::DebugResult, rd::RItemAXt, ud::UData};

impl RItemAXt {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attr_rid) = self.remote_resist_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
