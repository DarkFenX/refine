use crate::{
    ad::generator::{
        AdgSupport,
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
    fn get_item_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        self.locsrq_mods
            .iter()
            .map(|v| KeyPart::from_item_eid(v.skill_id))
            .collect()
    }
    fn get_group_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        self.locgroup_mods
            .iter()
            .map(|v| KeyPart::from_item_grp_eid(v.group_id))
            .collect()
    }
    fn get_attr_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let item_mods = &self.item_mods;
        let loc_mods = &self.loc_mods;
        let locgroup_mods = &self.locgroup_mods;
        let locsrq_mods = &self.locsrq_mods;
        let mut fks = Vec::with_capacity(item_mods.len() + loc_mods.len() + locgroup_mods.len() + locsrq_mods.len());
        fks.extend(item_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks.extend(loc_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks.extend(locgroup_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks.extend(locsrq_mods.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)));
        fks
    }
}
