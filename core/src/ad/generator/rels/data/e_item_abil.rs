use crate::{
    ad::generator::{
        GSupport, get_abil_effect,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemAbil,
};

impl Pk for EItemAbil {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id.into(), self.abil_id.into()]
    }
}

impl Fk for EItemAbil {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id.into()]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if let Some(effect_eid) = get_abil_effect(self.abil_id) {
            fks.push(effect_eid.into());
        }
        fks
    }
    fn get_abil_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.abil_id.into()]
    }
}
