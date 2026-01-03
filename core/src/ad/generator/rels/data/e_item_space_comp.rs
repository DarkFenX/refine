use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemSpaceComp,
};

impl Pk for EItemSpaceComp {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id.into()]
    }
}

impl Fk for EItemSpaceComp {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id.into()]
    }
    fn get_item_list_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        for buff_data in self.iter_data() {
            if let Some(item_list_eid) = buff_data.item_list_filter {
                fks.push(item_list_eid.into());
            }
        }
        fks
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        for buff_data in self.iter_data() {
            fks.extend(buff_data.buffs.iter().map(|v| KeyPart::from(v.id)));
        }
        fks
    }
}
