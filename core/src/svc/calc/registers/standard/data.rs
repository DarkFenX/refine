use crate::{
    ad::{AItemGrpId, AItemId, AItemListId},
    misc::{AttrSpec, EffectSpec},
    svc::calc::{CtxModifier, LocationKind, RawModifier},
    ud::{UFitKey, UItemKey},
    util::{RMapRSet, RSet},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct StandardRegister {
    // Items which are holders of a location kind (like char, ship)
    // Map<(affectee fit key, affectee location kind), affectee item keys>
    pub(super) affectee_root: RMapRSet<(UFitKey, LocationKind), UItemKey>,
    // Items belonging to certain fit and location kind (e.g. char's implants, ship's modules)
    // Map<(affectee fit key, affectee location kind), affectee item keys>
    pub(super) affectee_loc: RMapRSet<(UFitKey, LocationKind), UItemKey>,
    // Items belonging to certain fit, location kind and group
    // Map<(affectee fit key, affectee location kind, affectee agroup ID), affectee item keys>
    pub(super) affectee_loc_grp: RMapRSet<(UFitKey, LocationKind, AItemGrpId), UItemKey>,
    // Items belonging to certain fit and location kind, and having certain skill requirement
    // Map<(affectee fit key, affectee location kind, affectee srq aitem ID), affectee item keys>
    pub(super) affectee_loc_srq: RMapRSet<(UFitKey, LocationKind, AItemId), UItemKey>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Map<(affectee fit key, affectee srq aitem ID), affectee item keys>
    pub(super) affectee_own_srq: RMapRSet<(UFitKey, AItemId), UItemKey>,
    // Buff-modifiable items, which belong to certain fit and are on specific item list
    // Map<(affectee fit key, item list ID), affectee item keys>
    pub(super) affectee_buffable: RMapRSet<(UFitKey, AItemListId), UItemKey>,
    // Fits which have ships which are modifiable by buffs via specific item list
    // Map<item list ID, (fit key, ship key)>
    pub(super) affectee_buffable_ships: RMapRSet<AItemListId, (UFitKey, UItemKey, LocationKind)>,
    // All raw modifiers tracked by register
    // Map<affector effect spec, modifiers>
    pub(super) rmods_all: RMapRSet<EffectSpec, RawModifier>,
    // All projected raw modifiers tracked by register
    // Map<affector effect spec, modifiers>
    pub(super) rmods_proj: RMapRSet<EffectSpec, RawModifier>,
    // Fleet modifiers on a per-fit basis
    // Map<affector fit key, modifiers>
    pub(super) rmods_fleet: RMapRSet<UFitKey, RawModifier>,
    // System-wide system effect modifiers
    pub(super) rmods_sw_system: RSet<RawModifier>,
    // System-wide buff modifiers
    pub(super) rmods_sw_buff: RSet<RawModifier>,
    // Fit-wide buff modifiers
    pub(super) rmods_fw_buff: RMapRSet<UFitKey, RawModifier>,
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
            rmods_all: RMapRSet::new(),
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
pub(in crate::svc::calc) struct StandardRegisterCtxMods {
    // Modifiers which rely on an item-attribute pair value
    // Map<attr spec, modifiers>
    pub(super) by_aspec: RMapRSet<AttrSpec, CtxModifier>,
    // Modifiers which modify item directly
    // Map<affectee item key, modifiers>
    pub(super) direct: RMapRSet<UItemKey, CtxModifier>,
    // Modifiers which modify 'other' location are always stored here, regardless if they actually
    // modify something or not
    // Map<affector item key, modifiers>
    pub(super) other: RMapRSet<UItemKey, CtxModifier>,
    // All modifiers which modify root entities (via ship or character reference) are kept here
    // Map<(affectee fit key, affectee location kind), modifiers>
    pub(super) root: RMapRSet<(UFitKey, LocationKind), CtxModifier>,
    // Modifiers influencing all items belonging to certain fit and location kind
    // Map<(affectee fit key, affectee location kind), modifiers>
    pub(super) loc: RMapRSet<(UFitKey, LocationKind), CtxModifier>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Map<(affectee fit key, affectee location, affectee agroup ID), modifiers>
    pub(super) loc_grp: RMapRSet<(UFitKey, LocationKind, AItemGrpId), CtxModifier>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill
    // requirement
    // Map<(affectee fit key, affectee location, affectee srq aitem ID), modifiers>
    pub(super) loc_srq: RMapRSet<(UFitKey, LocationKind, AItemId), CtxModifier>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain
    // skill requirement
    // Map<(affectee fit key, affectee srq aitem ID), modifiers>
    pub(super) own_srq: RMapRSet<(UFitKey, AItemId), CtxModifier>,
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
pub(in crate::svc::calc) struct StandardRegisterRawProjStatus {
    // Valid item-targeted modifiers which target eligible item kind, with projectee item passing
    // all the checks
    // Map<projectee item ID, modifiers>
    pub(super) active: RMapRSet<UItemKey, RawModifier>,
    // Valid item-targeted modifiers which target eligible item kind, with projectee item failing
    // some checks, and thus modifiers being inactive
    // Map<projectee item ID, modifiers>
    pub(super) inactive: RMapRSet<UItemKey, RawModifier>,
}
impl StandardRegisterRawProjStatus {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            active: RMapRSet::new(),
            inactive: RMapRSet::new(),
        }
    }
}
