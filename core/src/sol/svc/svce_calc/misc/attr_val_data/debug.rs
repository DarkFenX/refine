use crate::sol::{
    svc::debug::{check_attr, check_item},
    SolDebugResult, SolView,
};

use super::SolAttrValData;

impl SolAttrValData {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for (item_id, item_data) in self.data.iter() {
            check_item(sol_view, item_id)?;
            // All calculated attributes are supposed to be available
            for attr_id in item_data.values.keys() {
                check_attr(sol_view, attr_id)?;
            }
        }
        Ok(())
    }
}
