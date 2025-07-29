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
        vec.extend(self.system_emitter_buffs.iter().map(|v| v.id));
        vec.extend(self.proxy_effect_buffs.iter().map(|v| v.id));
        vec.extend(self.proxy_trigger_buffs.iter().map(|v| v.id));
        vec.extend(self.ship_link_buffs.iter().map(|v| v.id));
        vec
    }
}
