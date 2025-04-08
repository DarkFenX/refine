use crate::sol::{
    debug::{DebugResult, check_fit_key, check_item_key},
    uad::Uad,
};

use super::Charge;

impl Charge {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        check_item_key(uad, self.get_cont_item_key(), false)?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
