use crate::{
    ad::{ABuff, ABuffModifier},
    ed::{EAttrId, EBuffId},
};

impl ABuff {
    pub(in crate::ad::generator::rels) fn iter_attr_eids(&self) -> impl Iterator<Item = EAttrId> {
        self.mods.iter().filter_map(|v| v.get_attr_eid())
    }
    pub(in crate::ad::generator::rels) fn iter_buff_eids(&self) -> impl Iterator<Item = EBuffId> {
        self.id.dc_eve().into_iter()
    }
}

impl ABuffModifier {
    fn get_attr_eid(&self) -> Option<EAttrId> {
        self.affectee_attr_id.dc_eve()
    }
}
