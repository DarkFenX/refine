use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemSpaceComp,
};

impl Pk for EItemSpaceComp {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id]
    }
}

impl Fk for EItemSpaceComp {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id]
    }
    fn get_item_list_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        for buff_data in self.iter_data() {
            if let Some(item_list_id) = buff_data.item_list_filter {
                vec.push(item_list_id);
            }
        }
        vec
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        for buff_data in self.iter_data() {
            vec.extend(buff_data.buffs.iter().map(|v| v.id));
        }
        vec
    }
}
