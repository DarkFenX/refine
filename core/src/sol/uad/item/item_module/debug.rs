use crate::sol::{
    uad::{item::debug, SolUad},
    SolDebugResult,
};

use super::SolModule;

impl SolModule {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        debug::check_fit(uad, &self.get_fit_id())?;
        if let Some(charge_id) = self.get_charge_id() {
            debug::check_item(uad, &charge_id)?;
        }
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
