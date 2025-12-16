use crate::{
    ad::{ABuff, ABuffAffecteeFilter, ABuffId, ABuffModifier},
    ed::{EAttrId, EBuffId, EItemGrpId},
};

impl ABuff {
    pub(in crate::ad::generator::rels) fn iter_e_group_ids(&self) -> impl Iterator<Item = EItemGrpId> {
        self.mods.iter().filter_map(|v| v.get_e_group_id())
    }
    pub(in crate::ad::generator::rels) fn iter_e_attr_ids(&self) -> impl Iterator<Item = EAttrId> {
        self.mods.iter().filter_map(|v| v.get_e_attr_id())
    }
    pub(in crate::ad::generator::rels) fn iter_e_buff_ids(&self) -> impl Iterator<Item = EBuffId> {
        self.id.dc_eve().into_iter()
    }
}

impl ABuffId {
    pub(super) fn dc_eve(&self) -> Option<EBuffId> {
        match self {
            ABuffId::Eve(eve_buff_id) => Some(*eve_buff_id),
            ABuffId::Custom(_) => None,
        }
    }
}

impl ABuffModifier {
    fn get_e_group_id(&self) -> Option<EItemGrpId> {
        self.affectee_filter.get_e_group_id()
    }
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        self.affectee_attr_id.dc_eve()
    }
}

impl ABuffAffecteeFilter {
    fn get_e_group_id(&self) -> Option<EItemGrpId> {
        match self {
            Self::Direct => None,
            Self::Loc => None,
            Self::LocGrp(group_id) => Some(*group_id),
            Self::LocSrq(_) => None,
        }
    }
}
