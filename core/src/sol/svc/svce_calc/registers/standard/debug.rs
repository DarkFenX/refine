use crate::{
    sol::{
        svc::{
            debug::{check_attr, check_effect, check_fit, check_item},
            svce_calc::debug::{check_ctx_modifier, check_raw_modifier},
        },
        SolView,
    },
    util::DebugResult,
};

use super::SolStandardRegister;

impl SolStandardRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for ((fit_id, _), item_ids) in self.affectee_root.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.affectee_loc.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.affectee_loc_grp.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.affectee_loc_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((fit_id, _), item_ids) in self.affectee_own_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for (fit_id, item_ids) in self.affectee_buffable.iter() {
            check_fit(sol_view, fit_id)?;
            for item_id in item_ids {
                check_item(sol_view, item_id)?;
            }
        }
        for ((item_id, effect_id), rmods) in self.rmods_nonproj.iter() {
            check_item(sol_view, item_id)?;
            check_effect(sol_view, effect_id)?;
            for rmod in rmods {
                check_raw_modifier(sol_view, rmod)?;
            }
        }
        for ((item_id, effect_id), rmods) in self.rmods_proj.iter() {
            check_item(sol_view, item_id)?;
            check_effect(sol_view, effect_id)?;
            for rmod in rmods {
                check_raw_modifier(sol_view, rmod)?;
            }
        }
        for (fit_id, rmods) in self.rmods_fleet.iter() {
            check_fit(sol_view, fit_id)?;
            for rmod in rmods {
                check_raw_modifier(sol_view, rmod)?;
            }
        }
        for rmod in self.rmods_sw_system.iter() {
            check_raw_modifier(sol_view, rmod)?;
        }
        for rmod in self.rmods_sw_buff.iter() {
            check_raw_modifier(sol_view, rmod)?;
        }
        for (fit_id, rmods) in self.rmods_fw_buff.iter() {
            check_fit(sol_view, fit_id)?;
            for rmod in rmods {
                check_raw_modifier(sol_view, rmod)?;
            }
        }
        for (attr_spec, cmods) in self.cmods_by_attr_spec.iter() {
            check_item(sol_view, &attr_spec.item_id)?;
            check_attr(sol_view, &attr_spec.attr_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        for (item_id, cmods) in self.cmods_direct.iter() {
            check_item(sol_view, item_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        for (item_id, cmods) in self.cmods_other.iter() {
            check_item(sol_view, item_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_root.iter() {
            check_fit(sol_view, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_loc.iter() {
            check_fit(sol_view, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        for ((fit_id, _, _), cmods) in self.cmods_loc_grp.iter() {
            check_fit(sol_view, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        for ((fit_id, _, _), cmods) in self.cmods_loc_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_own_srq.iter() {
            check_fit(sol_view, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(sol_view, cmod)?;
            }
        }
        Ok(())
    }
}
