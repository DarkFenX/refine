use crate::{
    adg::rels::{KeyPart, Pk},
    ed::EItemList,
};

impl Pk for EItemList {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

// No FK implementation, we don't care what data is in item lists for cleanup purposes, since it's
// used up during cache generation, and data in it is used just for membership checks, thus links
// from it are not interesting enough to restore linked data
