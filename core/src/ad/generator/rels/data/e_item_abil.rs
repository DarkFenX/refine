use crate::{
    ad::generator::{
        GSupport, get_abil_effect,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemAbil,
};

impl Pk for EItemAbil {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.item_id),
            KeyPart::from_abil_eid(self.abil_id),
        ]
    }
}

impl Fk for EItemAbil {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_item_eid(self.item_id)]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if let Some(effect_eid) = get_abil_effect(self.abil_id) {
            let fk = KeyPart::from_effect_eid(effect_eid);
            fks.push(fk);
        }
        fks
    }
    fn get_abil_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_abil_eid(self.abil_id)]
    }
}
