use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemSpaceComp,
};

impl Pk for EItemSpaceComp {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_item_eid(self.item_id)]
    }
}

impl Fk for EItemSpaceComp {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_item_eid(self.item_id)]
    }
    fn get_item_list_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        for buff_data in self.iter_data() {
            if let Some(item_list_eid) = buff_data.item_list_filter {
                let fk = KeyPart::from_item_list_eid(item_list_eid);
                fks.push(fk);
            }
        }
        fks
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        for buff_data in self.iter_data() {
            fks.extend(buff_data.buffs.iter().map(|e_entry| KeyPart::from_buff_eid(e_entry.id)));
        }
        fks
    }
}
