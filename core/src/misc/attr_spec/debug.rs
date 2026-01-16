use super::main::AttrSpec;
use crate::{dbg::DebugResult, ud::UData};

impl AttrSpec {
    pub(crate) fn consistency_check(&self, u_data: &UData, check_item_load: bool) -> DebugResult {
        self.item_uid.consistency_check(u_data, check_item_load)?;
        self.attr_rid.consistency_check(u_data)?;
        Ok(())
    }
}
