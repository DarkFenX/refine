use crate::sol::{
    debug::{DebugResult, check_a_effect_id, check_fit_id, check_item_key},
    svc::calc::debug::{check_ctx_modifier, check_raw_modifier},
    uad::Uad,
};

use super::StandardRegister;

impl StandardRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for ((fit_id, _), item_keys) in self.affectee_root.iter() {
            check_fit_id(uad, fit_id)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_id, _), item_keys) in self.affectee_loc.iter() {
            check_fit_id(uad, fit_id)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_id, _, _), item_keys) in self.affectee_loc_grp.iter() {
            check_fit_id(uad, fit_id)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_id, _, _), item_keys) in self.affectee_loc_srq.iter() {
            check_fit_id(uad, fit_id)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_id, _), item_keys) in self.affectee_own_srq.iter() {
            check_fit_id(uad, fit_id)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for (fit_id, item_keys) in self.affectee_buffable.iter() {
            check_fit_id(uad, fit_id)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((item_key, a_effect_id), rmods) in self.rmods_nonproj.iter() {
            check_item_key(uad, *item_key, true)?;
            check_a_effect_id(uad, a_effect_id)?;
            for rmod in rmods {
                check_raw_modifier(uad, rmod)?;
            }
        }
        for ((item_key, a_effect_id), rmods) in self.rmods_proj.iter() {
            check_item_key(uad, *item_key, true)?;
            check_a_effect_id(uad, a_effect_id)?;
            for rmod in rmods {
                check_raw_modifier(uad, rmod)?;
            }
        }
        for (fit_id, rmods) in self.rmods_fleet.iter() {
            check_fit_id(uad, fit_id)?;
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
            check_fit_id(uad, fit_id)?;
            for rmod in rmods {
                check_raw_modifier(uad, rmod)?;
            }
        }
        // Attributes of attr specs are not checked, because we do not verify if those do exist when
        // adding modifiers
        for (attr_spec, cmods) in self.cmods_by_attr_spec.iter() {
            check_item_key(uad, attr_spec.item_key, true)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for (item_key, cmods) in self.cmods_direct.iter() {
            check_item_key(uad, *item_key, true)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for (item_key, cmods) in self.cmods_other.iter() {
            check_item_key(uad, *item_key, true)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_root.iter() {
            check_fit_id(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_loc.iter() {
            check_fit_id(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _, _), cmods) in self.cmods_loc_grp.iter() {
            check_fit_id(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _, _), cmods) in self.cmods_loc_srq.iter() {
            check_fit_id(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        for ((fit_id, _), cmods) in self.cmods_own_srq.iter() {
            check_fit_id(uad, fit_id)?;
            for cmod in cmods {
                check_ctx_modifier(uad, cmod)?;
            }
        }
        Ok(())
    }
}
