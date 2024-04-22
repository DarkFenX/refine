use crate::{
    ss::{
        svc::debug::{check_fit, check_item},
        SsView,
    },
    util::DebugResult,
};

use super::SolAffecteeRegister;

impl SolAffecteeRegister {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for ((fit_id, _), item_ids) in self.root.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.loc.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.loc_grp.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.loc_srq.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.own_srq.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for (fit_id, item_ids) in self.buff_all.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        Ok(())
    }
}
