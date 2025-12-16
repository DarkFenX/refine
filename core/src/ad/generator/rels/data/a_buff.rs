use crate::{
    ad::{ABuff, ABuffModifier},
    ed::EAttrId,
};

impl ABuff {
    pub(in crate::ad::generator::rels) fn iter_e_attr_ids(&self) -> impl Iterator<Item = EAttrId> {
        self.mods.iter().filter_map(|v| v.get_e_attr_id())
    }
}

impl ABuffModifier {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        self.affectee_attr_id.dc_eve()
    }
}
