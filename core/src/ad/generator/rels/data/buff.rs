use crate::{
    ad::{
        ABuff, ABuffModifier,
        generator::{
            GSupport,
            rels::{Fk, KeyPart, Pk},
        },
    },
    ed::{EAttrId, EBuff},
};

impl Pk for EBuff {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for EBuff {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.locsrq_mods.iter().map(|v| v.skill_id).collect()
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.locgroup_mods.iter().map(|v| v.group_id).collect()
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        vec.extend(self.item_mods.iter().map(|v| v.attr_id));
        vec.extend(self.loc_mods.iter().map(|v| v.attr_id));
        vec.extend(self.locgroup_mods.iter().map(|v| v.attr_id));
        vec.extend(self.locsrq_mods.iter().map(|v| v.attr_id));
        vec
    }
}

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
