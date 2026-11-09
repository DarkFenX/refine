use super::raw_mod::{RawModAffector, RawModItemDependent};
use crate::{
    ad::{ABuffAffecteeFilter, AEffectAffecteeFilter, AItemGrpId, AItemId},
    svc::calc::Location,
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
    pub(super) fn try_from_effect_affectee_filter(
        affectee_filter: &AEffectAffecteeFilter,
        affector: RawModAffector,
    ) -> Result<Self, RawModItemDependent> {
        Ok(match affectee_filter {
            AEffectAffecteeFilter::Direct(loc) => Self::Direct(Location::from_a_effect_loc(*loc)),
            AEffectAffecteeFilter::Loc(loc) => Self::Loc(Location::from_a_effect_loc(*loc)),
            AEffectAffecteeFilter::LocGrp(loc, grp_id) => Self::LocGrp(Location::from_a_effect_loc(*loc), *grp_id),
            AEffectAffecteeFilter::LocSrq(loc, mod_srq) => {
                Self::LocSrq(Location::from_a_effect_loc(*loc), affector.get_srq_type_aid(mod_srq)?)
            }
            AEffectAffecteeFilter::OwnSrq(mod_srq) => Self::OwnSrq(affector.get_srq_type_aid(mod_srq)?),
        })
    }
    pub(super) fn try_from_buff_affectee_filter(
        buff_affectee_filter: &ABuffAffecteeFilter,
        loc: Location,
        affector: RawModAffector,
    ) -> Result<Self, RawModItemDependent> {
        Ok(match buff_affectee_filter {
            ABuffAffecteeFilter::Direct => Self::Direct(loc),
            ABuffAffecteeFilter::Loc => Self::Loc(loc),
            ABuffAffecteeFilter::LocGrp(grp_id) => Self::LocGrp(loc, *grp_id),
            ABuffAffecteeFilter::LocSrq(mod_srq) => Self::LocSrq(loc, affector.get_srq_type_aid(mod_srq)?),
        })
    }
}
