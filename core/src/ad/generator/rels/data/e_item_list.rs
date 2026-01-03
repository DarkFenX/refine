use crate::{
    ad::generator::rels::{Fk, KeyPart, Pk},
    ed::EItemList,
};

impl Pk for EItemList {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id.into()]
    }
}

// No actual implementation for item lists, since we do not plan to restore anything linked from
// item lists, and item lists themselves will be cleaned up during conversion
impl Fk for EItemList {}
