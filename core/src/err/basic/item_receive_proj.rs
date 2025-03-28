use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemReceiveProjError {
    pub item_id: ItemId,
    pub item_kind: &'static str,
}
impl std::error::Error for ItemReceiveProjError {}
impl std::fmt::Display for ItemReceiveProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} not found", self.item_kind, self.item_id)
    }
}
