use crate::sol::{
    svc::debug::{check_fit, check_item},
    uad::SolUad,
    SolDebugResult,
};

use super::{SolRest, SolRestFitData};

impl SolRest {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (fit_id, fit_data) in self.data.iter() {
            check_fit(uad, fit_id)?;
            fit_data.debug_consistency_check(uad)?;
        }
        Ok(())
    }
}

impl SolRestFitData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for item_id in self.mods_online.iter() {
            check_item(uad, item_id)?;
        }
        Ok(())
    }
}
