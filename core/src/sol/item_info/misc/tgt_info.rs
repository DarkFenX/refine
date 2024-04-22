use crate::defs::{AttrVal, SolItemId};

pub struct SolTgtInfo {
    pub item_id: SolItemId,
    pub range: Option<AttrVal>,
}
impl SolTgtInfo {
    pub(in crate::sol::item_info) fn new(item_id: SolItemId, range: Option<AttrVal>) -> Self {
        Self { item_id, range }
    }
}
