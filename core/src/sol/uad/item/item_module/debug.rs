use crate::sol::{
    debug::{SolDebugResult, check_fit, check_item},
    uad::SolUad,
};

use super::SolModule;

impl SolModule {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        check_fit(uad, &self.get_fit_id())?;
        if let Some(charge_id) = self.get_charge_id() {
            check_item(uad, &charge_id, false)?;
        }
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
