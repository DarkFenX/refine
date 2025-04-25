use crate::sol::{
    debug::{DebugResult, check_item_key},
    uad::Uad,
};

use super::AttrValData;

impl AttrValData {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for item_key in self.data.keys() {
            check_item_key(uad, *item_key, true)?;
        }
        Ok(())
    }
}
