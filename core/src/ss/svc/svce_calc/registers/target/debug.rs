use crate::{
    ss::{
        svc::debug::{check_fit, check_item},
        SsView,
    },
    util::DebugResult,
};

use super::SsTargetRegister;

impl SsTargetRegister {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for ((fit_id, _), item_ids) in self.tgts_root.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.tgts_loc.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.tgts_loc_grp.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.tgts_loc_srq.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.tgts_own_srq.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        for (fit_id, item_ids) in self.tgts_buff_all.iter() {
            check_fit(ss_view, fit_id)?;
            for item_id in item_ids {
                check_item(ss_view, item_id)?;
            }
        }
        Ok(())
    }
}
