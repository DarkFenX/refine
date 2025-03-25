use crate::sol::{
    debug::{DebugResult, check_fit_id, check_item_id},
    uad::Uad,
};

use super::Charge;

impl Charge {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_id(uad, &self.get_fit_id())?;
        check_item_id(uad, &self.get_cont_item_id(), false)?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
