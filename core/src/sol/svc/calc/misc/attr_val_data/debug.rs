use crate::sol::{
    debug::{SolDebugResult, check_attr, check_item},
    uad::SolUad,
};

use super::SolAttrValData;

impl SolAttrValData {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (item_id, item_data) in self.data.iter() {
            check_item(uad, item_id, true)?;
            // All calculated attributes are supposed to be available
            for attr_id in item_data.values.keys() {
                check_attr(uad, attr_id)?;
            }
        }
        Ok(())
    }
}
