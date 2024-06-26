use crate::{
    defs::{EEffectId, EItemGrpId, EItemId, SolFitId, SolItemId},
    sol::svc::svce_calc::{SolAttrSpec, SolCtxModifier, SolLocationKind, SolRawModifier},
    util::{StMapSetL1, StSet},
};

pub(in crate::sol::svc::svce_calc) struct SolStandardRegister {
    // Items which are holders of a location kind (like char, ship)
    // Map<(affectee fit ID, affectee location kind), affectee item IDs>
    pub(super) affectee_root: StMapSetL1<(SolFitId, SolLocationKind), SolItemId>,
    // Items belonging to certain fit and location kind (e.g. char's implants, ship's modules)
    // Map<(affectee fit ID, affectee location kind), affectee item IDs>
    pub(super) affectee_loc: StMapSetL1<(SolFitId, SolLocationKind), SolItemId>,
    // Items belonging to certain fit, location kind and group
    // Map<(affectee fit ID, affectee location kind, affectee group ID), affectee item IDs>
    pub(super) affectee_loc_grp: StMapSetL1<(SolFitId, SolLocationKind, EItemGrpId), SolItemId>,
    // Items belonging to certain fit and location kind, and having certain skill requirement
    // Map<(affectee fit ID, affectee location kind, affectee skillreq type ID), affectee item IDs>
    pub(super) affectee_loc_srq: StMapSetL1<(SolFitId, SolLocationKind, EItemId), SolItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Map<(affectee fit ID, affectee skillreq type ID), affectee item IDs>
    pub(super) affectee_own_srq: StMapSetL1<(SolFitId, EItemId), SolItemId>,
    // Everything-buff-modifiable items which belong to certain fit
    // Map<affectee fit ID, affectee item IDs>
    pub(super) affectee_buffable: StMapSetL1<SolFitId, SolItemId>,
    // All non-projected raw modifiers tracked by register
    // Map<(affector item ID, affector effect ID), modifiers>
    pub(super) rmods_nonproj: StMapSetL1<(SolItemId, EEffectId), SolRawModifier>,
    // All projected raw modifiers tracked by register
    // Map<(affector item ID, affector effect ID), modifiers>
    pub(super) rmods_proj: StMapSetL1<(SolItemId, EEffectId), SolRawModifier>,
    // Fleet modifiers on a per-fit basis
    // Map<affector fit ID, modifiers>
    pub(super) rmods_fleet: StMapSetL1<SolFitId, SolRawModifier>,
    // System-wide system effect modifiers
    pub(super) rmods_sw_system: StSet<SolRawModifier>,
    // System-wide buff modifiers
    pub(super) rmods_sw_buff: StSet<SolRawModifier>,
    // Fit-wide buff modifiers
    pub(super) rmods_fw_buff: StMapSetL1<SolFitId, SolRawModifier>,
    // Modifiers which rely on an item-attribute pair value
    // Map<attr spec, modifiers>
    pub(super) cmods_by_attr_spec: StMapSetL1<SolAttrSpec, SolCtxModifier>,
    // Modifiers which modify item directly
    // Map<affectee item ID, modifiers>
    pub(super) cmods_direct: StMapSetL1<SolItemId, SolCtxModifier>,
    // Modifiers which modify 'other' domain are always stored here, regardless if they actually
    // modify something or not
    // Map<affector item ID, modifiers>
    pub(super) cmods_other: StMapSetL1<SolItemId, SolCtxModifier>,
    // All modifiers which modify root entities (via ship or character reference) are kept here
    // Map<(affectee fit ID, affectee location kind), modifiers>
    pub(super) cmods_root: StMapSetL1<(SolFitId, SolLocationKind), SolCtxModifier>,
    // Modifiers influencing all items belonging to certain fit and location kind
    // Map<(affectee fit ID, affectee location kind), modifiers>
    pub(super) cmods_loc: StMapSetL1<(SolFitId, SolLocationKind), SolCtxModifier>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Map<(affectee fit ID, affectee location, affectee group ID), modifiers>
    pub(super) cmods_loc_grp: StMapSetL1<(SolFitId, SolLocationKind, EItemGrpId), SolCtxModifier>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill
    // requirement
    // Map<(affectee fit ID, affectee location, affectee skillreq type ID), modifiers>
    pub(super) cmods_loc_srq: StMapSetL1<(SolFitId, SolLocationKind, EItemId), SolCtxModifier>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain
    // skill requirement
    // Map<(affectee fit ID, affectee skillreq type ID), modifiers>
    pub(super) cmods_own_srq: StMapSetL1<(SolFitId, EItemId), SolCtxModifier>,
}
impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            affectee_root: StMapSetL1::new(),
            affectee_loc: StMapSetL1::new(),
            affectee_loc_grp: StMapSetL1::new(),
            affectee_loc_srq: StMapSetL1::new(),
            affectee_own_srq: StMapSetL1::new(),
            affectee_buffable: StMapSetL1::new(),
            cmods_by_attr_spec: StMapSetL1::new(),
            cmods_direct: StMapSetL1::new(),
            cmods_other: StMapSetL1::new(),
            cmods_root: StMapSetL1::new(),
            cmods_loc: StMapSetL1::new(),
            cmods_loc_grp: StMapSetL1::new(),
            cmods_loc_srq: StMapSetL1::new(),
            cmods_own_srq: StMapSetL1::new(),
            rmods_fleet: StMapSetL1::new(),
            rmods_sw_system: StSet::new(),
            rmods_sw_buff: StSet::new(),
            rmods_fw_buff: StMapSetL1::new(),
            rmods_nonproj: StMapSetL1::new(),
            rmods_proj: StMapSetL1::new(),
        }
    }
}
