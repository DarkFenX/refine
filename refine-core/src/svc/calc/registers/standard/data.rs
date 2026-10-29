use crate::{
    ad::{AItemGrpId, AItemId},
    misc::{AttrSpec, EffectSpec},
    rd::{RAttrId, RItemListId},
    svc::calc::{CtxModKey, LocationKind, RawModId},
    ud::{UFitId, UItemId},
    util::{RMapRSet, RSet},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct StandardRegister {
    // Items which are holders of a location kind (like char, ship)
    // Map<(affectee fit, affectee location kind), affectee items>
    pub(super) affectee_root: RMapRSet<(UFitId, LocationKind), UItemId>,
    // Items belonging to certain fit and location kind (e.g. char's implants, ship's modules)
    // Map<(affectee fit, affectee location kind), affectee items>
    pub(super) affectee_loc: RMapRSet<(UFitId, LocationKind), UItemId>,
    // Items belonging to certain fit, location kind and group
    // Map<(affectee fit, affectee location kind, affectee group), affectee items>
    pub(super) affectee_loc_grp: RMapRSet<(UFitId, LocationKind, AItemGrpId), UItemId>,
    // Items belonging to certain fit and location kind, and having certain skill requirement
    // Map<(affectee fit, affectee location kind, affectee srq), affectee items>
    pub(super) affectee_loc_srq: RMapRSet<(UFitId, LocationKind, AItemId), UItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Map<(affectee fit, affectee srq), affectee item>
    pub(super) affectee_own_srq: RMapRSet<(UFitId, AItemId), UItemId>,
    // Buff-modifiable items, which belong to certain fit and are on specific item list
    // Map<(affectee fit, item list), affectee item>
    pub(super) affectee_buffable: RMapRSet<(UFitId, RItemListId), UItemId>,
    // Fits which have ships which are modifiable by buffs via specific item list
    // Map<item list, (fit, ship)>
    pub(super) affectee_buffable_ships: RMapRSet<RItemListId, (UFitId, UItemId, LocationKind)>,
    // All projected raw modifiers tracked by register
    // Map<affector effect spec, modifiers>
    pub(super) rmods_proj: RMapRSet<EffectSpec, RawModId>,
    // Fleet modifiers on a per-fit basis
    // Map<affector fit, modifiers>
    pub(super) rmods_fleet: RMapRSet<UFitId, RawModId>,
    // System-wide system effect modifiers
    pub(super) rmods_sw_system: RSet<RawModId>,
    // System-wide buff modifiers
    pub(super) rmods_sw_buff: RSet<RawModId>,
    // Fit-wide buff modifiers
    pub(super) rmods_fw_buff: RMapRSet<UFitId, RawModId>,
    // Child containers
    pub(super) rmods_proj_status: StandardRegisterRawProjStatus,
    pub(super) cmods: StandardRegisterCtxMods,
}
impl StandardRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            affectee_root: RMapRSet::new(),
            affectee_loc: RMapRSet::new(),
            affectee_loc_grp: RMapRSet::new(),
            affectee_loc_srq: RMapRSet::new(),
            affectee_own_srq: RMapRSet::new(),
            affectee_buffable: RMapRSet::new(),
            affectee_buffable_ships: RMapRSet::new(),
            rmods_proj: RMapRSet::new(),
            rmods_fleet: RMapRSet::new(),
            rmods_sw_system: RSet::new(),
            rmods_sw_buff: RSet::new(),
            rmods_fw_buff: RMapRSet::new(),
            rmods_proj_status: StandardRegisterRawProjStatus::new(),
            cmods: StandardRegisterCtxMods::new(),
        }
    }
}

#[derive(Clone)]
pub(super) struct StandardRegisterCtxMods {
    // Modifiers which rely on an item-attribute pair value
    // Map<affector attr spec, modifiers>
    pub(super) by_aspec: RMapRSet<AttrSpec, CtxModKey>,
    // Modifiers which modify item directly
    // Map<(affectee item, affectee attr), modifiers>
    pub(super) direct: RMapRSet<(UItemId, RAttrId), CtxModKey>,
    // Modifiers which modify 'other' location are always stored here, regardless if they actually
    // modify something or not
    // Map<(affector item, affectee attr), modifiers>
    pub(super) other: RMapRSet<(UItemId, RAttrId), CtxModKey>,
    // All modifiers which modify root entities (via ship or character reference) are kept here
    // Map<(affectee fit, affectee location kind, affectee attr), modifiers>
    pub(super) root: RMapRSet<(UFitId, LocationKind, RAttrId), CtxModKey>,
    // Modifiers influencing all items belonging to certain fit and location kind
    // Map<(affectee fit, affectee location kind, affectee attr), modifiers>
    pub(super) loc: RMapRSet<(UFitId, LocationKind, RAttrId), CtxModKey>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Map<(affectee fit, affectee location, affectee group, affectee attr), modifiers>
    pub(super) loc_grp: RMapRSet<(UFitId, LocationKind, AItemGrpId, RAttrId), CtxModKey>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill
    // requirement
    // Map<(affectee fit, affectee location, affectee srq, affectee attr), modifiers>
    pub(super) loc_srq: RMapRSet<(UFitId, LocationKind, AItemId, RAttrId), CtxModKey>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain
    // skill requirement
    // Map<(affectee fit, affectee srq, affectee attr), modifiers>
    pub(super) own_srq: RMapRSet<(UFitId, AItemId, RAttrId), CtxModKey>,
}
impl StandardRegisterCtxMods {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            by_aspec: RMapRSet::new(),
            direct: RMapRSet::new(),
            other: RMapRSet::new(),
            root: RMapRSet::new(),
            loc: RMapRSet::new(),
            loc_grp: RMapRSet::new(),
            loc_srq: RMapRSet::new(),
            own_srq: RMapRSet::new(),
        }
    }
}

#[derive(Clone)]
pub(super) struct StandardRegisterRawProjStatus {
    // Valid item-targeted modifiers which target eligible item kind, with projectee item passing
    // all the checks
    // Map<projectee item, modifiers>
    pub(super) active: RMapRSet<UItemId, RawModId>,
    // Valid item-targeted modifiers which target eligible item kind, with projectee item failing
    // some checks, and thus modifiers being inactive
    // Map<projectee item, modifiers>
    pub(super) inactive: RMapRSet<UItemId, RawModId>,
}
impl StandardRegisterRawProjStatus {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            active: RMapRSet::new(),
            inactive: RMapRSet::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, svc::calc::registers::RawModStorage, ud::UData};

    impl StandardRegister {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData, rmods: &RawModStorage) -> DebugResult {
            let Self {
                affectee_root,
                affectee_loc,
                affectee_loc_grp,
                affectee_loc_srq,
                affectee_own_srq,
                affectee_buffable,
                affectee_buffable_ships,
                rmods_proj,
                rmods_fleet,
                rmods_sw_system,
                rmods_sw_buff,
                rmods_fw_buff,
                rmods_proj_status,
                cmods,
            } = self;
            for ((fit_uid, _), item_uids) in affectee_root.iter() {
                fit_uid.consistency_check(u_data)?;
                for item_uid in item_uids {
                    item_uid.consistency_check(u_data, true)?;
                }
            }
            for ((fit_uid, _), item_uids) in affectee_loc.iter() {
                fit_uid.consistency_check(u_data)?;
                for item_uid in item_uids {
                    item_uid.consistency_check(u_data, true)?;
                }
            }
            for ((fit_uid, _, _), item_uids) in affectee_loc_grp.iter() {
                fit_uid.consistency_check(u_data)?;
                for item_uid in item_uids {
                    item_uid.consistency_check(u_data, true)?;
                }
            }
            for ((fit_uid, _, _), item_uids) in affectee_loc_srq.iter() {
                fit_uid.consistency_check(u_data)?;
                for item_uid in item_uids {
                    item_uid.consistency_check(u_data, true)?;
                }
            }
            for ((fit_uid, _), item_uids) in affectee_own_srq.iter() {
                fit_uid.consistency_check(u_data)?;
                for item_uid in item_uids {
                    item_uid.consistency_check(u_data, true)?;
                }
            }
            for ((fit_uid, _), item_uids) in affectee_buffable.iter() {
                fit_uid.consistency_check(u_data)?;
                for item_uid in item_uids {
                    item_uid.consistency_check(u_data, true)?;
                }
            }
            for afectee_infos in affectee_buffable_ships.values() {
                for (fit_uid, item_uid, _) in afectee_infos {
                    fit_uid.consistency_check(u_data)?;
                    item_uid.consistency_check(u_data, true)?;
                }
            }
            for (espec, rmod_ids) in rmods_proj.iter() {
                espec.consistency_check(u_data, true)?;
                for &rmod_id in rmod_ids {
                    rmod_id.consistency_check(u_data, rmods)?;
                }
            }
            for (fit_uid, rmod_ids) in rmods_fleet.iter() {
                fit_uid.consistency_check(u_data)?;
                for &rmod_id in rmod_ids {
                    rmod_id.consistency_check(u_data, rmods)?;
                }
            }
            for &rmod_id in rmods_sw_system.iter() {
                rmod_id.consistency_check(u_data, rmods)?;
            }
            for &rmod_id in rmods_sw_buff.iter() {
                rmod_id.consistency_check(u_data, rmods)?;
            }
            for (fit_uid, rmod_ids) in rmods_fw_buff.iter() {
                fit_uid.consistency_check(u_data)?;
                for &rmod_id in rmod_ids {
                    rmod_id.consistency_check(u_data, rmods)?;
                }
            }
            for (&projectee_uid, rmod_ids) in rmods_proj_status.active.iter() {
                // Projectees don't have to be loaded for an entry to be added here
                projectee_uid.consistency_check(u_data, false)?;
                for &rmod_id in rmod_ids {
                    rmod_id.consistency_check(u_data, rmods)?;
                }
            }
            for (&projectee_uid, rmod_ids) in rmods_proj_status.inactive.iter() {
                // Projectees don't have to be loaded for an entry to be added here
                projectee_uid.consistency_check(u_data, false)?;
                for &rmod_id in rmod_ids {
                    rmod_id.consistency_check(u_data, rmods)?;
                }
            }
            // Attributes of attr specs are not checked, because we do not verify if those do exist
            // when adding modifiers
            for (aspec, cmods) in cmods.by_aspec.iter() {
                aspec.consistency_check(u_data, true)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            for ((item_uid, _), cmods) in cmods.direct.iter() {
                // Sometimes direct modifications can target non-loaded items (e.g. drones)
                item_uid.consistency_check(u_data, false)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            for ((item_uid, _), cmods) in cmods.other.iter() {
                item_uid.consistency_check(u_data, true)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            for ((fit_uid, _, _), cmods) in cmods.root.iter() {
                fit_uid.consistency_check(u_data)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            for ((fit_uid, _, _), cmods) in cmods.loc.iter() {
                fit_uid.consistency_check(u_data)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            for ((fit_uid, _, _, _), cmods) in cmods.loc_grp.iter() {
                fit_uid.consistency_check(u_data)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            for ((fit_uid, _, _, _), cmods) in cmods.loc_srq.iter() {
                fit_uid.consistency_check(u_data)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            for ((fit_uid, _, _), cmods) in cmods.own_srq.iter() {
                fit_uid.consistency_check(u_data)?;
                for ckey in cmods {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            Ok(())
        }
    }
}
