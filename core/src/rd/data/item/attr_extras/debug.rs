use crate::{
    dbg::{DebugResult, check_attr_id},
    rd::RItemAXt,
    ud::UData,
};

impl RItemAXt {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attr_key) = self.remote_resist_attr_key {
            check_attr_id(u_data, attr_key)?;
        }
        Ok(())
    }
}
