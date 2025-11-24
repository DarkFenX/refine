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
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        for buff_data in self.iter_data().filter_map(|v| v.as_ref()) {
            vec.extend(buff_data.buffs.iter().map(|v| v.id));
        }
        vec
    }
}
