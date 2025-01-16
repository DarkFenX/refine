use crate::sol::{
    debug::{check_effect, check_fit, check_item, SolDebugResult},
    svc::calc::debug::{check_ctx_modifier, check_raw_modifier},
    uad::SolUad,
};

use super::SolStandardRegister;

impl SolStandardRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for ((fit_id, _), item_ids) in self.affectee_root.iter() {
            check_fit(uad, fit_id)?;
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for ((fit_id, _), item_ids) in self.affectee_loc.iter() {
            check_fit(uad, fit_id)?;
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.affectee_loc_grp.iter() {
            check_fit(uad, fit_id)?;
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for ((fit_id, _, _), item_ids) in self.affectee_loc_srq.iter() {
            check_fit(uad, fit_id)?;
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for ((fit_id, _), item_ids) in self.affectee_own_srq.iter() {
            check_fit(uad, fit_id)?;
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for (fit_id, item_ids) in self.affectee_buffable.iter() {
            check_fit(uad, fit_id)?;
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for ((item_id, effect_id), rmods) in self.rmods_nonproj.iter() {
            check_item(uad, item_id, true)?;
            check_effect(uad, effect_id)?;
            for rmod in rmods {
                check_raw_modifier(uad, rmod)?;
            }
        }
        for ((item_id, effect_id), rmods) in self.rmods_proj.iter() {
            check_item(uad, item_id, true)?;
            check_effect(uad, effect_id)?;
            for rmod in rmods {
                check_raw_modifier(uad, rmod)?;
            }
        }
        for (fit_id, rmods) in self.rmods_fleet.iter() {
            check_fit(uad, fit_id)?;
            for rmod in rmods {
                check_raw_modifier(uad, rmod)?;
            }
        }
        for rmod in self.rmods_sw_system.iter() {
            check_raw_modifier(uad, rmod)?;
        }
        for rmod in self.rmods_sw_buff.iter() {
            check_raw_modifier(uad, rmod)?;
        }
        for (fit_id, rmods) in self.rmods_fw_buff.iter() {
            check_fit(uad, fit_id)?;
            for rmod in rmods {
                check_raw_modifier(uad, rmod)?;
            }
        }
        // Attributes of attr specs are not checked, because we do not verify if those do exist when
        // adding modifiers
        for (attr_spec, cmods) in self.cmods_by_attr_spec.iter() {
            check_item(uad, &attr_spec.item_id, true)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for (item_id, cmods) in self.cmods_direct.iter() {
            check_item(uad, item_id, true)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for (item_id, cmods) in self.cmods_other.iter() {
            check_item(uad, item_id, true)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_root.iter() {
            check_fit(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_loc.iter() {
            check_fit(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _, _), cmods) in self.cmods_loc_grp.iter() {
            check_fit(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _, _), cmods) in self.cmods_loc_srq.iter() {
            check_fit(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_own_srq.iter() {
            check_fit(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        Ok(())
    }
}
