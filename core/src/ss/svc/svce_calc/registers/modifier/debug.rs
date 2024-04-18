use crate::{
    ss::{
        svc::{
            debug::{check_fit, check_item},
            svce_calc::debug::check_modifier,
        },
        SsView,
    },
    util::DebugResult,
};

use super::ModifierRegister;

impl ModifierRegister {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (item_id, ss_mods) in self.mods.iter() {
            check_item(ss_view, item_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for (item_id, ss_mods) in self.mods_direct.iter() {
            check_item(ss_view, item_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for ((fit_id, _), ss_mods) in self.mods_toploc.iter() {
            check_fit(ss_view, fit_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for (item_id, ss_mods) in self.mods_other.iter() {
            check_item(ss_view, item_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for ((fit_id, _), ss_mods) in self.mods_parloc.iter() {
            check_fit(ss_view, fit_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for ((fit_id, _, _), ss_mods) in self.mods_parloc_grp.iter() {
            check_fit(ss_view, fit_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for ((fit_id, _, _), ss_mods) in self.mods_parloc_srq.iter() {
            check_fit(ss_view, fit_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for ((fit_id, _), ss_mods) in self.mods_own_srq.iter() {
            check_fit(ss_view, fit_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for (fit_id, ss_mods) in self.mods_buff_all.iter() {
            check_fit(ss_view, fit_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for (fit_id, ss_mods) in self.mods_fleet_fit.iter() {
            check_fit(ss_view, fit_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        for ss_mod in self.sw_mods.iter() {
            check_modifier(ss_view, ss_mod)?;
        }
        Ok(())
    }
}
