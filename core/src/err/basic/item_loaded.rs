use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemLoadedError {
    pub item_id: ItemId,
}
impl ItemLoadedError {
    pub(crate) fn new(item_id: ItemId) -> Self {
        Self { item_id }
    }
}
impl std::error::Error for ItemLoadedError {}
impl std::fmt::Display for ItemLoadedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} is not loaded", self.item_id)
    }
}
