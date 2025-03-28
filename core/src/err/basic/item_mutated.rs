use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemMutatedError {
    pub item_id: ItemId,
}
impl std::error::Error for ItemMutatedError {}
impl std::fmt::Display for ItemMutatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} is not mutated", self.item_id)
    }
}
