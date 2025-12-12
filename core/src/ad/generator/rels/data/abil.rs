use crate::{
    ad::generator::{
        GSupport, get_abil_effect,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EFighterAbil,
};

impl Pk for EFighterAbil {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for EFighterAbil {
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(v) = get_abil_effect(self.id) {
            vec.push(v);
        }
        vec
    }
}
