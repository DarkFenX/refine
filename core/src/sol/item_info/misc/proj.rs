use crate::defs::{AttrVal, SolItemId};

pub struct SolProjInfo {
    pub item_id: SolItemId,
    pub range: Option<AttrVal>,
}
impl SolProjInfo {
    pub(in crate::sol::item_info) fn new(item_id: SolItemId, range: Option<AttrVal>) -> Self {
        Self { item_id, range }
    }
}
