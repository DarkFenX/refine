use crate::sol::{
    debug::{DebugResult, check_fit_id, check_item_id},
    uad::Uad,
};

use super::Module;

impl Module {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_id(uad, &self.get_fit_id())?;
        if let Some(charge_id) = self.get_charge_item_id() {
            check_item_id(uad, &charge_id, false)?;
        }
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
