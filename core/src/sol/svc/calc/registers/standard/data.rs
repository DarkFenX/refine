use crate::{
    ad,
    sol::{
        FitId, ItemId,
        svc::calc::{AttrSpec, CtxModifier, LocationKind, RawModifier},
    },
    util::{StMapSetL1, StSet},
};

#[derive(Clone)]
pub(in crate::sol::svc::calc) struct StandardRegister {
    // Items which are holders of a location kind (like char, ship)
    // Map<(affectee fit ID, affectee location kind), affectee item IDs>
    pub(super) affectee_root: StMapSetL1<(FitId, LocationKind), ItemId>,
    // Items belonging to certain fit and location kind (e.g. char's implants, ship's modules)
    // Map<(affectee fit ID, affectee location kind), affectee item IDs>
    pub(super) affectee_loc: StMapSetL1<(FitId, LocationKind), ItemId>,
    // Items belonging to certain fit, location kind and group
    // Map<(affectee fit ID, affectee location kind, affectee agroup ID), affectee item IDs>
    pub(super) affectee_loc_grp: StMapSetL1<(FitId, LocationKind, ad::AItemGrpId), ItemId>,
    // Items belonging to certain fit and location kind, and having certain skill requirement
    // Map<(affectee fit ID, affectee location kind, affectee skillreq aitem ID), affectee item IDs>
    pub(super) affectee_loc_srq: StMapSetL1<(FitId, LocationKind, ad::AItemId), ItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Map<(affectee fit ID, affectee skillreq aitem ID), affectee item IDs>
    pub(super) affectee_own_srq: StMapSetL1<(FitId, ad::AItemId), ItemId>,
    // Everything-buff-modifiable items which belong to certain fit
    // Map<affectee fit ID, affectee item IDs>
    pub(super) affectee_buffable: StMapSetL1<FitId, ItemId>,
    // All non-projected raw modifiers tracked by register
    // Map<(affector item ID, affector aeffect ID), modifiers>
    pub(super) rmods_nonproj: StMapSetL1<(ItemId, ad::AEffectId), RawModifier>,
    // All projected raw modifiers tracked by register
    // Map<(affector item ID, affector aeffect ID), modifiers>
    pub(super) rmods_proj: StMapSetL1<(ItemId, ad::AEffectId), RawModifier>,
    // Fleet modifiers on a per-fit basis
    // Map<affector fit ID, modifiers>
    pub(super) rmods_fleet: StMapSetL1<FitId, RawModifier>,
    // System-wide system effect modifiers
    pub(super) rmods_sw_system: StSet<RawModifier>,
    // System-wide buff modifiers
    pub(super) rmods_sw_buff: StSet<RawModifier>,
    // Fit-wide buff modifiers
    pub(super) rmods_fw_buff: StMapSetL1<FitId, RawModifier>,
    // Modifiers which rely on an item-attribute pair value
    // Map<attr spec, modifiers>
    pub(super) cmods_by_attr_spec: StMapSetL1<AttrSpec, CtxModifier>,
    // Modifiers which modify item directly
    // Map<affectee item ID, modifiers>
    pub(super) cmods_direct: StMapSetL1<ItemId, CtxModifier>,
    // Modifiers which modify 'other' location are always stored here, regardless if they actually
    // modify something or not
    // Map<affector item ID, modifiers>
    pub(super) cmods_other: StMapSetL1<ItemId, CtxModifier>,
    // All modifiers which modify root entities (via ship or character reference) are kept here
    // Map<(affectee fit ID, affectee location kind), modifiers>
    pub(super) cmods_root: StMapSetL1<(FitId, LocationKind), CtxModifier>,
    // Modifiers influencing all items belonging to certain fit and location kind
    // Map<(affectee fit ID, affectee location kind), modifiers>
    pub(super) cmods_loc: StMapSetL1<(FitId, LocationKind), CtxModifier>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Map<(affectee fit ID, affectee location, affectee agroup ID), modifiers>
    pub(super) cmods_loc_grp: StMapSetL1<(FitId, LocationKind, ad::AItemGrpId), CtxModifier>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill
    // requirement
    // Map<(affectee fit ID, affectee location, affectee skillreq aitem ID), modifiers>
    pub(super) cmods_loc_srq: StMapSetL1<(FitId, LocationKind, ad::AItemId), CtxModifier>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain
    // skill requirement
    // Map<(affectee fit ID, affectee skillreq aitem ID), modifiers>
    pub(super) cmods_own_srq: StMapSetL1<(FitId, ad::AItemId), CtxModifier>,
}
impl StandardRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            affectee_root: StMapSetL1::new(),
            affectee_loc: StMapSetL1::new(),
            affectee_loc_grp: StMapSetL1::new(),
            affectee_loc_srq: StMapSetL1::new(),
            affectee_own_srq: StMapSetL1::new(),
            affectee_buffable: StMapSetL1::new(),
            rmods_nonproj: StMapSetL1::new(),
            rmods_proj: StMapSetL1::new(),
            rmods_fleet: StMapSetL1::new(),
            rmods_sw_system: StSet::new(),
            rmods_sw_buff: StSet::new(),
            rmods_fw_buff: StMapSetL1::new(),
            cmods_by_attr_spec: StMapSetL1::new(),
            cmods_direct: StMapSetL1::new(),
            cmods_other: StMapSetL1::new(),
            cmods_root: StMapSetL1::new(),
            cmods_loc: StMapSetL1::new(),
            cmods_loc_grp: StMapSetL1::new(),
            cmods_loc_srq: StMapSetL1::new(),
            cmods_own_srq: StMapSetL1::new(),
        }
    }
}
