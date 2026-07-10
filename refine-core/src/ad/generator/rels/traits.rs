use crate::ad::generator::{AdgSupport, rels::KeyPart};

pub(in crate::ad::generator) trait Pk {
    fn get_pk(&self) -> Vec<KeyPart>;
}

pub(in crate::ad::generator) trait Fk {
    fn get_item_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_group_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_item_list_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_attr_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_effect_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_abil_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_buff_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
}
