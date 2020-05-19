use serde;
use serde_json;

use crate::defines::ReeInt;
use crate::dh;

pub(super) trait Assemble<T> {
    fn assemble(&self, id: ReeInt) -> T;
}

#[derive(Debug)]
pub(super) struct FsdItem {
    pub(super) id: String,
    pub(super) item: serde_json::Value,
}
impl FsdItem {
    pub fn new<T: Into<String>>(id: T, item: serde_json::Value) -> FsdItem {
        FsdItem { id: id.into(), item }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveType {
    pub(super) typeID: ReeInt,
    pub(super) groupID: ReeInt,
}
impl Assemble<dh::EveType> for EveType {
    fn assemble(&self, id: ReeInt) -> dh::EveType {
        dh::EveType::new(id, self.groupID)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveGroup {
    pub(super) groupID: ReeInt,
    pub(super) categoryID: ReeInt,
}
impl Assemble<dh::EveGroup> for EveGroup {
    fn assemble(&self, id: ReeInt) -> dh::EveGroup {
        dh::EveGroup::new(id, self.categoryID)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
