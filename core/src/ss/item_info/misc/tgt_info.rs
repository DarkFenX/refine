use crate::defs::{AttrVal, SsItemId};

pub struct SsTgtInfo {
    pub item_id: SsItemId,
    pub range: Option<AttrVal>,
}
impl SsTgtInfo {
    pub(in crate::ss::item_info) fn new(item_id: SsItemId, range: Option<AttrVal>) -> Self {
        Self { item_id, range }
    }
}
