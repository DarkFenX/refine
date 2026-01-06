use crate::{
    ad::{ABuff, ABuffAffecteeFilter, ABuffId, ABuffModifier},
    ed::{EAttrId, EBuffId, EItemGrpId},
};

impl ABuff {
    pub(in crate::ad::generator::rels) fn iter_group_eids(&self) -> impl Iterator<Item = EItemGrpId> {
        self.mods.iter().filter_map(|v| v.get_group_eid())
    }
    pub(in crate::ad::generator::rels) fn iter_attr_eids(&self) -> impl Iterator<Item = EAttrId> {
        self.mods.iter().filter_map(|v| v.get_attr_eid())
    }
    pub(in crate::ad::generator::rels) fn iter_buff_eids(&self) -> impl Iterator<Item = EBuffId> {
        self.id.dc_eve().into_iter()
    }
}

impl ABuffId {
    pub(super) fn dc_eve(&self) -> Option<EBuffId> {
        match self {
            ABuffId::Eve(eve_buff_aid) => Some(EBuffId::from_i32(eve_buff_aid.into_i32())),
            ABuffId::Custom(_) => None,
        }
    }
}

impl ABuffModifier {
    fn get_group_eid(&self) -> Option<EItemGrpId> {
        self.affectee_filter.get_group_eid()
    }
    fn get_attr_eid(&self) -> Option<EAttrId> {
        self.affectee_attr_id.dc_eve()
    }
}

impl ABuffAffecteeFilter {
    fn get_group_eid(&self) -> Option<EItemGrpId> {
        match self {
            Self::Direct => None,
            Self::Loc => None,
            Self::LocGrp(group_aid) => Some(EItemGrpId::from_i32(group_aid.into_i32())),
            Self::LocSrq(_) => None,
        }
    }
}
