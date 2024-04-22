use crate::{
    sol::{
        svc::debug::{check_fit, check_item},
        SolView,
    },
    util::DebugResult,
};

use super::SolAffecteeRegister;

impl SolAffecteeRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for ((fit_id, _), item_ids) in self.root.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.loc.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.loc_grp.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.loc_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.own_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for (fit_id, item_ids) in self.buff_all.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        Ok(())
    }
}
