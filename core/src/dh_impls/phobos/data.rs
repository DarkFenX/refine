use crate::defines::ReeInt;
use crate::dh;
use serde;

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
