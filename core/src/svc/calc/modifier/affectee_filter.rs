use crate::{
    ad::{ABuffAffecteeFilter, AEffectAffecteeFilter, AItemGrpId, AItemId, AModifierSrq},
    svc::calc::Location,
    ud::UItem,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AffecteeFilter {
    Direct(Location),
    Loc(Location),
    LocGrp(Location, AItemGrpId),
    LocSrq(Location, AItemId),
    OwnSrq(AItemId),
}
impl AffecteeFilter {
    pub(super) fn from_effect_affectee_filter(affectee_filter: &AEffectAffecteeFilter, item: &UItem) -> Self {
        match affectee_filter {
            AEffectAffecteeFilter::Direct(loc) => Self::Direct(Location::from_a_effect_loc(*loc)),
            AEffectAffecteeFilter::Loc(loc) => Self::Loc(Location::from_a_effect_loc(*loc)),
            AEffectAffecteeFilter::LocGrp(loc, grp_id) => Self::LocGrp(Location::from_a_effect_loc(*loc), *grp_id),
            AEffectAffecteeFilter::LocSrq(loc, mod_srq) => {
                Self::LocSrq(Location::from_a_effect_loc(*loc), get_srq_type_id(mod_srq, item))
            }
            AEffectAffecteeFilter::OwnSrq(mod_srq) => Self::OwnSrq(get_srq_type_id(mod_srq, item)),
        }
    }
    pub(super) fn from_buff_affectee_filter(
        buff_affectee_filter: &ABuffAffecteeFilter,
        loc: Location,
        item: &UItem,
    ) -> Self {
        match buff_affectee_filter {
            ABuffAffecteeFilter::Direct => Self::Direct(loc),
            ABuffAffecteeFilter::Loc => Self::Loc(loc),
            ABuffAffecteeFilter::LocGrp(grp_id) => Self::LocGrp(loc, *grp_id),
            ABuffAffecteeFilter::LocSrq(mod_srq) => Self::LocSrq(loc, get_srq_type_id(mod_srq, item)),
        }
    }
}

fn get_srq_type_id(mod_srq: &AModifierSrq, u_item: &UItem) -> AItemId {
    match mod_srq {
        AModifierSrq::SelfRef => u_item.get_type_id(),
        AModifierSrq::ItemId(type_id) => *type_id,
    }
}
