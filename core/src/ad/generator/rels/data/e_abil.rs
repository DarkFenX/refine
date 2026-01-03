use crate::{
    ad::generator::{
        GSupport, get_abil_effect,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EAbil,
};

impl Pk for EAbil {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id.into()]
    }
}

impl Fk for EAbil {
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if let Some(effect_eid) = get_abil_effect(self.id) {
            fks.push(effect_eid.into());
        }
        fks
    }
}
