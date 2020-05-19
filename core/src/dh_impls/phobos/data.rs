use serde;

use crate::defines::ReeInt;
use crate::dh;

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveType {
    pub(super) typeID: ReeInt,
    pub(super) groupID: ReeInt,
}
impl Into<dh::EveType> for EveType {
    fn into(self) -> dh::EveType {
        dh::EveType::new(self.typeID, self.groupID)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct EveGroup {
    pub(super) groupID: ReeInt,
    pub(super) categoryID: ReeInt,
}
impl Into<dh::EveGroup> for EveGroup {
    fn into(self) -> dh::EveGroup {
        dh::EveGroup::new(self.groupID, self.categoryID)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
