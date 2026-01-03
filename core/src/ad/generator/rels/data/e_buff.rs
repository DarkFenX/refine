use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EBuff,
};

impl Pk for EBuff {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id.into()]
    }
}

impl Fk for EBuff {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.locsrq_mods.iter().map(|v| v.skill_id.into()).collect()
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.locgroup_mods.iter().map(|v| v.group_id.into()).collect()
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        fks.extend(self.item_mods.iter().map(|v| KeyPart::from(v.attr_id)));
        fks.extend(self.loc_mods.iter().map(|v| KeyPart::from(v.attr_id)));
        fks.extend(self.locgroup_mods.iter().map(|v| KeyPart::from(v.attr_id)));
        fks.extend(self.locsrq_mods.iter().map(|v| KeyPart::from(v.attr_id)));
        fks
    }
}
