use crate::{
    ad,
    misc::{AttrSpec, EffectSpec},
    svc::calc::{CtxModifier, LocationKind, RawModifier},
    uad::{UadFitKey, UadItemKey},
    util::{RMapRSet, RSet},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct StandardRegister {
    // Items which are holders of a location kind (like char, ship)
    // Map<(affectee fit key, affectee location kind), affectee item keys>
    pub(super) affectee_root: RMapRSet<(UadFitKey, LocationKind), UadItemKey>,
    // Items belonging to certain fit and location kind (e.g. char's implants, ship's modules)
    // Map<(affectee fit key, affectee location kind), affectee item keys>
    pub(super) affectee_loc: RMapRSet<(UadFitKey, LocationKind), UadItemKey>,
    // Items belonging to certain fit, location kind and group
    // Map<(affectee fit key, affectee location kind, affectee agroup ID), affectee item keys>
    pub(super) affectee_loc_grp: RMapRSet<(UadFitKey, LocationKind, ad::AItemGrpId), UadItemKey>,
    // Items belonging to certain fit and location kind, and having certain skill requirement
    // Map<(affectee fit key, affectee location kind, affectee srq aitem ID), affectee item keys>
    pub(super) affectee_loc_srq: RMapRSet<(UadFitKey, LocationKind, ad::AItemId), UadItemKey>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Map<(affectee fit key, affectee srq aitem ID), affectee item keys>
    pub(super) affectee_own_srq: RMapRSet<(UadFitKey, ad::AItemId), UadItemKey>,
    // Everything-buff-modifiable items which belong to certain fit
    // Map<affectee fit key, affectee item keys>
    pub(super) affectee_buffable: RMapRSet<UadFitKey, UadItemKey>,
    // All raw modifiers tracked by register
    // Map<affector effect spec, modifiers>
    pub(super) rmods_all: RMapRSet<EffectSpec, RawModifier>,
    // All projected raw modifiers tracked by register
    // Map<affector effect spec, modifiers>
    pub(super) rmods_proj: RMapRSet<EffectSpec, RawModifier>,
    // Fleet modifiers on a per-fit basis
    // Map<affector fit key, modifiers>
    pub(super) rmods_fleet: RMapRSet<UadFitKey, RawModifier>,
    // System-wide system effect modifiers
    pub(super) rmods_sw_system: RSet<RawModifier>,
    // System-wide direct buff modifiers
    pub(super) rmods_sw_buff_direct: RSet<RawModifier>,
    // System-wide indirect buff modifiers
    pub(super) rmods_sw_buff_indirect: RSet<RawModifier>,
    // Fit-wide direct buff modifiers
    pub(super) rmods_fw_buff_direct: RMapRSet<UadFitKey, RawModifier>,
    // Fit-wide indirect buff modifiers
    pub(super) rmods_fw_buff_indirect: RMapRSet<UadFitKey, RawModifier>,
    // Valid item-targeted modifiers which target eligible item kind, with projectee item passing
    // all the checks
    // Map<projectee item ID, modifiers>
    pub(super) rmods_proj_active: RMapRSet<UadItemKey, RawModifier>,
    // Valid item-targeted modifiers which target eligible item kind, with projectee item failing
    // some checks, and thus modifiers being inactive
    // Map<projectee item ID, modifiers>
    pub(super) rmods_proj_inactive: RMapRSet<UadItemKey, RawModifier>,
    // Modifiers which rely on an item-attribute pair value
    // Map<attr spec, modifiers>
    pub(super) cmods_by_aspec: RMapRSet<AttrSpec, CtxModifier>,
    // Modifiers which modify item directly
    // Map<affectee item key, modifiers>
    pub(super) cmods_direct: RMapRSet<UadItemKey, CtxModifier>,
    // Modifiers which modify 'other' location are always stored here, regardless if they actually
    // modify something or not
    // Map<affector item key, modifiers>
    pub(super) cmods_other: RMapRSet<UadItemKey, CtxModifier>,
    // All modifiers which modify root entities (via ship or character reference) are kept here
    // Map<(affectee fit key, affectee location kind), modifiers>
    pub(super) cmods_root: RMapRSet<(UadFitKey, LocationKind), CtxModifier>,
    // Modifiers influencing all items belonging to certain fit and location kind
    // Map<(affectee fit key, affectee location kind), modifiers>
    pub(super) cmods_loc: RMapRSet<(UadFitKey, LocationKind), CtxModifier>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Map<(affectee fit key, affectee location, affectee agroup ID), modifiers>
    pub(super) cmods_loc_grp: RMapRSet<(UadFitKey, LocationKind, ad::AItemGrpId), CtxModifier>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill
    // requirement
    // Map<(affectee fit key, affectee location, affectee srq aitem ID), modifiers>
    pub(super) cmods_loc_srq: RMapRSet<(UadFitKey, LocationKind, ad::AItemId), CtxModifier>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain
    // skill requirement
    // Map<(affectee fit key, affectee srq aitem ID), modifiers>
    pub(super) cmods_own_srq: RMapRSet<(UadFitKey, ad::AItemId), CtxModifier>,
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
            rmods_all: RMapRSet::new(),
            rmods_proj: RMapRSet::new(),
            rmods_fleet: RMapRSet::new(),
            rmods_sw_system: RSet::new(),
            rmods_sw_buff_direct: RSet::new(),
            rmods_sw_buff_indirect: RSet::new(),
            rmods_fw_buff_direct: RMapRSet::new(),
            rmods_fw_buff_indirect: RMapRSet::new(),
            rmods_proj_active: RMapRSet::new(),
            rmods_proj_inactive: RMapRSet::new(),
            cmods_by_aspec: RMapRSet::new(),
            cmods_direct: RMapRSet::new(),
            cmods_other: RMapRSet::new(),
            cmods_root: RMapRSet::new(),
            cmods_loc: RMapRSet::new(),
            cmods_loc_grp: RMapRSet::new(),
            cmods_loc_srq: RMapRSet::new(),
            cmods_own_srq: RMapRSet::new(),
        }
    }
}
