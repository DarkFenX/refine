use crate::adg::{rels::KeyPart, GSupport};

pub(in crate::adg) trait Pk {
    fn get_pk(&self) -> Vec<KeyPart>;
}

pub(in crate::adg) trait Fk {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_abil_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        Vec::new()
    }
}
