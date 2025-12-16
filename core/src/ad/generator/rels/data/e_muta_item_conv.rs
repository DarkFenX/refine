use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EMutaItemConv,
};

impl Pk for EMutaItemConv {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.muta_id, self.in_item_id]
    }
}

impl Fk for EMutaItemConv {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.muta_id, self.in_item_id, self.out_item_id]
    }
}
