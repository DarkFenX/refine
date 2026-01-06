use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EMutaItemConv,
};

impl Pk for EMutaItemConv {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.muta_id),
            KeyPart::from_item_eid(self.in_item_id),
        ]
    }
}

impl Fk for EMutaItemConv {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.muta_id),
            KeyPart::from_item_eid(self.in_item_id),
            KeyPart::from_item_eid(self.out_item_id),
        ]
    }
}
