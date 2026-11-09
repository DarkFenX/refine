use crate::{
    ad::{ABuffAffecteeFilter, AEffectAffecteeFilter, AItemGrpId, AItemId, AModifierSrq},
    svc::calc::Location,
    ud::UItem,
};

#[cfg_attr(feature = "dev", derive(PartialEq))]
#[derive(Copy, Clone)]
pub(in crate::svc::calc) enum AffecteeFilter {
    Direct(Location),
    Loc(Location),
    LocGrp(Location, AItemGrpId),
    LocSrq(Location, AItemId),
    OwnSrq(AItemId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AffecteeFilter {
    /// Fails if filter needs an item and it was not passed.
    pub(super) fn try_from_effect_affectee_filter(
        affectee_filter: &AEffectAffecteeFilter,
        item: Option<&UItem>,
    ) -> Option<Self> {
        Some(match affectee_filter {
            AEffectAffecteeFilter::Direct(loc) => Self::Direct(Location::from_a_effect_loc(*loc)),
            AEffectAffecteeFilter::Loc(loc) => Self::Loc(Location::from_a_effect_loc(*loc)),
            AEffectAffecteeFilter::LocGrp(loc, grp_id) => Self::LocGrp(Location::from_a_effect_loc(*loc), *grp_id),
            AEffectAffecteeFilter::LocSrq(loc, mod_srq) => {
                Self::LocSrq(Location::from_a_effect_loc(*loc), try_get_srq_type_aid(mod_srq, item)?)
            }
            AEffectAffecteeFilter::OwnSrq(mod_srq) => Self::OwnSrq(try_get_srq_type_aid(mod_srq, item)?),
        })
    }
    /// Fails if filter needs an item and it was not passed.
    pub(super) fn try_from_buff_affectee_filter(
        buff_affectee_filter: &ABuffAffecteeFilter,
        loc: Location,
        item: Option<&UItem>,
    ) -> Option<Self> {
        Some(match buff_affectee_filter {
            ABuffAffecteeFilter::Direct => Self::Direct(loc),
            ABuffAffecteeFilter::Loc => Self::Loc(loc),
            ABuffAffecteeFilter::LocGrp(grp_id) => Self::LocGrp(loc, *grp_id),
            ABuffAffecteeFilter::LocSrq(mod_srq) => Self::LocSrq(loc, try_get_srq_type_aid(mod_srq, item)?),
        })
    }
}

fn try_get_srq_type_aid(mod_srq: &AModifierSrq, u_item: Option<&UItem>) -> Option<AItemId> {
    match mod_srq {
        AModifierSrq::SelfRef => u_item.map(|u_item| u_item.get_rendered_type_aid()),
        AModifierSrq::ItemId(type_aid) => Some(*type_aid),
    }
}
