use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemLoadedError {
    pub item_id: ItemId,
}
impl std::error::Error for ItemLoadedError {}
impl std::fmt::Display for ItemLoadedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} is not loaded", self.item_id)
    }
}
