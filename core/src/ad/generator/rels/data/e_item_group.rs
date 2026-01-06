use crate::{
    ad::generator::rels::{Fk, KeyPart, Pk},
    ed::EItemGroup,
};

impl Pk for EItemGroup {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_item_grp_eid(self.id)]
    }
}

impl Fk for EItemGroup {}
