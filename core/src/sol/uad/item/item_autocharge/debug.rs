use crate::sol::{
    debug::{DebugResult, check_fit_key, check_item_key},
    uad::Uad,
};

use super::Autocharge;

impl Autocharge {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        // All autocharges are supposed to be loaded
        check_item_key(uad, self.get_cont_item_key(), true)?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
