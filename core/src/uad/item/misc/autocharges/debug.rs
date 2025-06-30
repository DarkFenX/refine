use crate::{
    dbg::{DebugResult, check_item_key},
    uad::{Uad, item::misc::Autocharges},
};

impl Autocharges {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for autocharge_key in self.values() {
            // All autocharges are supposed to be loaded
            check_item_key(uad, *autocharge_key, true)?;
        }
        Ok(())
    }
}
