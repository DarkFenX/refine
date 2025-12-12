use crate::{
    ad::gnr::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EBuff,
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
