use crate::sol::{
    svc::debug::{check_fit, check_item},
    SolDebugResult, SolView,
};

use super::{SolSvcRestatData, SolSvcRestatFitData};

impl SolSvcRestatData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for (fit_id, fit_data) in self.data.iter() {
            check_fit(sol_view, fit_id)?;
            fit_data.debug_consistency_check(sol_view)?;
        }
        Ok(())
    }
}

impl SolSvcRestatFitData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for item_id in self.mods_online.iter() {
            check_item(sol_view, item_id)?;
        }
        Ok(())
    }
}
