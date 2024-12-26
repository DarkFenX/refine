use crate::sol::{
    svc::debug::{check_attr, check_item},
    SolDebugResult, SolView,
};

use super::SolRahSim;

impl SolRahSim {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for (item_id, item_data) in self.resonances.iter() {
            check_item(sol_view, item_id)?;
            for attr_id in item_data.keys() {
                check_attr(sol_view, attr_id)?;
            }
        }
        Ok(())
    }
}
