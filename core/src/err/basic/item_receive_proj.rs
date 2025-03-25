use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemReceiveProjError {
    pub item_id: ItemId,
    pub item_kind: &'static str,
}
impl ItemReceiveProjError {
    pub(crate) fn new(item_id: ItemId, item_kind: &'static str) -> Self {
        Self { item_id, item_kind }
    }
}
impl std::error::Error for ItemReceiveProjError {}
impl std::fmt::Display for ItemReceiveProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} not found", self.item_kind, self.item_id)
    }
}
