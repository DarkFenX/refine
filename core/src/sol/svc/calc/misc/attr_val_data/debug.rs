use crate::sol::{
    debug::{DebugResult, check_a_attr_id, check_item_key},
    uad::Uad,
};

use super::AttrValData;

impl AttrValData {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (item_key, item_data) in self.data.iter() {
            check_item_key(uad, *item_key, true)?;
            // All calculated attributes are supposed to be available
            for a_attr_id in item_data.values.keys() {
                check_a_attr_id(uad, a_attr_id)?;
            }
        }
        Ok(())
    }
}
