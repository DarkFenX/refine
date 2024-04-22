use crate::{
    sol::{
        svc::debug::{check_attr, check_item},
        SolView,
    },
    util::DebugResult,
};

use super::SolAttrValData;

impl SolAttrValData {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (item_id, attr_map) in self.data.iter() {
            check_item(sol_view, item_id)?;
            // All attributes are supposed to be available too
            for attr_id in attr_map.keys() {
                check_attr(sol_view, attr_id)?;
            }
        }
        Ok(())
    }
}
