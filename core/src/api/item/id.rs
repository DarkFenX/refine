use std::num::Wrapping;

use crate::util::{LibDefault, LibIncrement};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ItemId(u32);
impl ItemId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    pub fn into_inner(self) -> u32 {
        self.0
    }
}
impl LibDefault for ItemId {
    fn lib_default() -> Self {
        Self(0)
    }
}
impl LibIncrement for ItemId {
    fn lib_increment(&mut self) {
        self.0 = (Wrapping(self.0) + Wrapping(1)).0;
    }
}
