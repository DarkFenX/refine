use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemNotMutatedError {
    pub item_id: ItemId,
}
impl std::error::Error for ItemNotMutatedError {}
impl std::fmt::Display for ItemNotMutatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} is mutated", self.item_id)
    }
}
