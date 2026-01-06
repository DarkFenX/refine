use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EBuff,
};

impl Pk for EBuff {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_buff_eid(self.id)]
    }
}

impl Fk for EBuff {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.locsrq_mods
            .iter()
            .map(|v| KeyPart::from_item_eid(v.skill_id))
            .collect()
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.locgroup_mods
            .iter()
            .map(|v| KeyPart::from_item_grp_eid(v.group_id))
            .collect()
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        fks.extend(self.item_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks.extend(self.loc_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks.extend(self.locgroup_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks.extend(self.locsrq_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks
    }
}
