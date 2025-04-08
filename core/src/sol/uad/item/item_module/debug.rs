use crate::sol::{
    debug::{DebugResult, check_fit_id, check_item_key},
    uad::Uad,
};

use super::Module;

impl Module {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_id(uad, &self.get_fit_id())?;
        if let Some(charge_key) = self.get_charge_item_key() {
            check_item_key(uad, charge_key, false)?;
        }
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
