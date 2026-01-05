use super::StandardRegister;
use crate::{
    dbg::{DebugResult, check_attr_rid, check_effect_rid, check_fit_uid, check_item_uid},
    svc::calc::debug::{check_cmod, check_rmod},
    ud::UData,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for ((fit_key, _), item_keys) in self.affectee_root.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for item_key in item_keys {
                check_item_uid(u_data, *item_key, true)?;
            }
        }
        for ((fit_key, _), item_keys) in self.affectee_loc.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for item_key in item_keys {
                check_item_uid(u_data, *item_key, true)?;
            }
        }
        for ((fit_key, _, _), item_keys) in self.affectee_loc_grp.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for item_key in item_keys {
                check_item_uid(u_data, *item_key, true)?;
            }
        }
        for ((fit_key, _, _), item_keys) in self.affectee_loc_srq.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for item_key in item_keys {
                check_item_uid(u_data, *item_key, true)?;
            }
        }
        for ((fit_key, _), item_keys) in self.affectee_own_srq.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for item_key in item_keys {
                check_item_uid(u_data, *item_key, true)?;
            }
        }
        for ((fit_key, _), item_keys) in self.affectee_buffable.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for item_key in item_keys {
                check_item_uid(u_data, *item_key, true)?;
            }
        }
        for afectee_infos in self.affectee_buffable_ships.values() {
            for (fit_key, item_key, _) in afectee_infos {
                check_fit_uid(u_data, *fit_key)?;
                check_item_uid(u_data, *item_key, true)?;
            }
        }
        for (espec, rmods) in self.rmods_all.iter() {
            check_item_uid(u_data, espec.item_uid, true)?;
            check_effect_rid(u_data, espec.effect_rid)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        for (espec, rmods) in self.rmods_proj.iter() {
            check_item_uid(u_data, espec.item_uid, true)?;
            check_effect_rid(u_data, espec.effect_rid)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        for (fit_key, rmods) in self.rmods_fleet.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        for rmod in self.rmods_sw_system.iter() {
            check_rmod(u_data, rmod)?;
        }
        for rmod in self.rmods_sw_buff.iter() {
            check_rmod(u_data, rmod)?;
        }
        for (fit_key, rmods) in self.rmods_fw_buff.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        for (&projectee_key, rmods) in self.rmods_proj_status.active.iter() {
            // Projectees don't have to be loaded for an entry to be added here
            check_item_uid(u_data, projectee_key, false)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        for (&projectee_key, rmods) in self.rmods_proj_status.inactive.iter() {
            // Projectees don't have to be loaded for an entry to be added here
            check_item_uid(u_data, projectee_key, false)?;
            for rmod in rmods {
                check_rmod(u_data, rmod)?;
            }
        }
        // Attributes of attr specs are not checked, because we do not verify if those do exist when
        // adding modifiers
        for (aspec, cmods) in self.cmods.by_aspec.iter() {
            check_item_uid(u_data, aspec.item_key, true)?;
            check_attr_rid(u_data, aspec.attr_key)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        for (item_key, cmods) in self.cmods.direct.iter() {
            // Sometimes direct modifications can target non-loaded items (e.g. drones)
            check_item_uid(u_data, *item_key, false)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        for (item_key, cmods) in self.cmods.other.iter() {
            check_item_uid(u_data, *item_key, true)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        for ((fit_key, _), cmods) in self.cmods.root.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        for ((fit_key, _), cmods) in self.cmods.loc.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        for ((fit_key, _, _), cmods) in self.cmods.loc_grp.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        for ((fit_key, _, _), cmods) in self.cmods.loc_srq.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        for ((fit_key, _), cmods) in self.cmods.own_srq.iter() {
            check_fit_uid(u_data, *fit_key)?;
            for cmod in cmods {
                check_cmod(u_data, cmod)?;
            }
        }
        Ok(())
    }
}
