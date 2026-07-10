use super::StandardRegister;
use crate::{dbg::DebugResult, ud::UData};

impl StandardRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for ((fit_uid, _), item_uids) in self.affectee_root.iter() {
            fit_uid.consistency_check(u_data)?;
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for ((fit_uid, _), item_uids) in self.affectee_loc.iter() {
            fit_uid.consistency_check(u_data)?;
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for ((fit_uid, _, _), item_uids) in self.affectee_loc_grp.iter() {
            fit_uid.consistency_check(u_data)?;
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for ((fit_uid, _, _), item_uids) in self.affectee_loc_srq.iter() {
            fit_uid.consistency_check(u_data)?;
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for ((fit_uid, _), item_uids) in self.affectee_own_srq.iter() {
            fit_uid.consistency_check(u_data)?;
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for ((fit_uid, _), item_uids) in self.affectee_buffable.iter() {
            fit_uid.consistency_check(u_data)?;
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for afectee_infos in self.affectee_buffable_ships.values() {
            for (fit_uid, item_uid, _) in afectee_infos {
                fit_uid.consistency_check(u_data)?;
                item_uid.consistency_check(u_data, true)?;
            }
        }
        for (espec, rmods) in self.rmods_all.iter() {
            espec.consistency_check(u_data, true)?;
            for rmod in rmods {
                rmod.consistency_check(u_data)?;
            }
        }
        for (espec, rmods) in self.rmods_proj.iter() {
            espec.consistency_check(u_data, true)?;
            for rmod in rmods {
                rmod.consistency_check(u_data)?;
            }
        }
        for (fit_uid, rmods) in self.rmods_fleet.iter() {
            fit_uid.consistency_check(u_data)?;
            for rmod in rmods {
                rmod.consistency_check(u_data)?;
            }
        }
        for rmod in self.rmods_sw_system.iter() {
            rmod.consistency_check(u_data)?;
        }
        for rmod in self.rmods_sw_buff.iter() {
            rmod.consistency_check(u_data)?;
        }
        for (fit_uid, rmods) in self.rmods_fw_buff.iter() {
            fit_uid.consistency_check(u_data)?;
            for rmod in rmods {
                rmod.consistency_check(u_data)?;
            }
        }
        for (&projectee_uid, rmods) in self.rmods_proj_status.active.iter() {
            // Projectees don't have to be loaded for an entry to be added here
            projectee_uid.consistency_check(u_data, false)?;
            for rmod in rmods {
                rmod.consistency_check(u_data)?;
            }
        }
        for (&projectee_uid, rmods) in self.rmods_proj_status.inactive.iter() {
            // Projectees don't have to be loaded for an entry to be added here
            projectee_uid.consistency_check(u_data, false)?;
            for rmod in rmods {
                rmod.consistency_check(u_data)?;
            }
        }
        // Attributes of attr specs are not checked, because we do not verify if those do exist when
        // adding modifiers
        for (aspec, cmods) in self.cmods.by_aspec.iter() {
            aspec.consistency_check(u_data, true)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        for (item_uid, cmods) in self.cmods.direct.iter() {
            // Sometimes direct modifications can target non-loaded items (e.g. drones)
            item_uid.consistency_check(u_data, false)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        for (item_uid, cmods) in self.cmods.other.iter() {
            item_uid.consistency_check(u_data, true)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        for ((fit_uid, _), cmods) in self.cmods.root.iter() {
            fit_uid.consistency_check(u_data)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        for ((fit_uid, _), cmods) in self.cmods.loc.iter() {
            fit_uid.consistency_check(u_data)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        for ((fit_uid, _, _), cmods) in self.cmods.loc_grp.iter() {
            fit_uid.consistency_check(u_data)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        for ((fit_uid, _, _), cmods) in self.cmods.loc_srq.iter() {
            fit_uid.consistency_check(u_data)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        for ((fit_uid, _), cmods) in self.cmods.own_srq.iter() {
            fit_uid.consistency_check(u_data)?;
            for cmod in cmods {
                cmod.consistency_check(u_data)?;
            }
        }
        Ok(())
    }
}
