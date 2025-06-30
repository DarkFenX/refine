use super::AttrValData;
use crate::{
    dbg::{DebugResult, check_item_key},
    uad::Uad,
};

impl AttrValData {
    pub(in crate::svc::calc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for item_key in self.data.keys() {
            check_item_key(uad, *item_key, true)?;
        }
        Ok(())
    }
}
