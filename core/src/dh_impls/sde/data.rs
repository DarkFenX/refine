use serde;
use serde_yaml;

use crate::defines::ReeInt;
use crate::dh;

pub(super) trait Assemble<T> {
    fn assemble(&self, id: ReeInt) -> T;
}

#[derive(Debug)]
pub(super) struct FsdItem {
    pub(super) id: serde_yaml::Value,
    pub(super) item: serde_yaml::Value,
}
impl FsdItem {
    pub fn new(id: serde_yaml::Value, item: serde_yaml::Value) -> FsdItem {
        FsdItem { id, item }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveType {
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
    pub(super) categoryID: ReeInt,
}
impl Assemble<dh::EveGroup> for EveGroup {
    fn assemble(&self, id: ReeInt) -> dh::EveGroup {
        dh::EveGroup::new(id, self.categoryID)
    }
}
