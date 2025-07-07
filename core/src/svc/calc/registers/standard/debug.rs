use super::StandardRegister;
use crate::{
    dbg::{DebugResult, check_a_effect_id, check_fit_key, check_item_key},
    svc::calc::debug::{check_cmod, check_rmod},
    uad::Uad,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for ((fit_key, _), item_keys) in self.affectee_root.iter() {
            check_fit_key(uad, *fit_key)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_key, _), item_keys) in self.affectee_loc.iter() {
            check_fit_key(uad, *fit_key)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_key, _, _), item_keys) in self.affectee_loc_grp.iter() {
            check_fit_key(uad, *fit_key)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_key, _, _), item_keys) in self.affectee_loc_srq.iter() {
            check_fit_key(uad, *fit_key)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for ((fit_key, _), item_keys) in self.affectee_own_srq.iter() {
            check_fit_key(uad, *fit_key)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for (fit_key, item_keys) in self.affectee_buffable.iter() {
            check_fit_key(uad, *fit_key)?;
            for item_key in item_keys {
                check_item_key(uad, *item_key, true)?;
            }
        }
        for (espec, rmods) in self.rmods_all.iter() {
            check_item_key(uad, espec.item_key, true)?;
            check_a_effect_id(uad, &espec.a_effect_id)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        for (espec, rmods) in self.rmods_proj.iter() {
            check_item_key(uad, espec.item_key, true)?;
            check_a_effect_id(uad, &espec.a_effect_id)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        for (fit_key, rmods) in self.rmods_fleet.iter() {
            check_fit_key(uad, *fit_key)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        for rmod in self.rmods_sw_system.iter() {
            check_rmod(uad, rmod)?;
        }
        for rmod in self.rmods_sw_buff_direct.iter() {
            check_rmod(uad, rmod)?;
        }
        for rmod in self.rmods_sw_buff_indirect.iter() {
            check_rmod(uad, rmod)?;
        }
        for (fit_key, rmods) in self.rmods_fw_buff_direct.iter() {
            check_fit_key(uad, *fit_key)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        for (fit_key, rmods) in self.rmods_fw_buff_indirect.iter() {
            check_fit_key(uad, *fit_key)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        for (&projectee_key, rmods) in self.rmods_proj_active.iter() {
            // Projectees don't have to be loaded for an entry to be added here
            check_item_key(uad, projectee_key, false)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        for (&projectee_key, rmods) in self.rmods_proj_inactive.iter() {
            // Projectees don't have to be loaded for an entry to be added here
            check_item_key(uad, projectee_key, false)?;
            for rmod in rmods {
                check_rmod(uad, rmod)?;
            }
        }
        // Attributes of attr specs are not checked, because we do not verify if those do exist when
        // adding modifiers
        for (aspec, cmods) in self.cmods_by_aspec.iter() {
            check_item_key(uad, aspec.item_key, true)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        for (item_key, cmods) in self.cmods_direct.iter() {
            // TODO: if/when typelist-filtered buffs are added, and direct modification processing
            // TODO: of non-ship entities is added, item key should be always loaded again
            // Sometimes direct modifications can target non-loaded items (e.g. drones)
            check_item_key(uad, *item_key, false)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        for (item_key, cmods) in self.cmods_other.iter() {
            check_item_key(uad, *item_key, true)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        for ((fit_key, _), cmods) in self.cmods_root.iter() {
            check_fit_key(uad, *fit_key)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        for ((fit_key, _), cmods) in self.cmods_loc.iter() {
            check_fit_key(uad, *fit_key)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        for ((fit_key, _, _), cmods) in self.cmods_loc_grp.iter() {
            check_fit_key(uad, *fit_key)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        for ((fit_key, _, _), cmods) in self.cmods_loc_srq.iter() {
            check_fit_key(uad, *fit_key)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        for ((fit_key, _), cmods) in self.cmods_own_srq.iter() {
            check_fit_key(uad, *fit_key)?;
            for cmod in cmods {
                check_cmod(uad, cmod)?;
            }
        }
        Ok(())
    }
}
