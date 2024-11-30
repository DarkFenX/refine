use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemMutatedError {
    pub item_id: SolItemId,
}
impl ItemMutatedError {
    pub(crate) fn new(item_id: SolItemId) -> Self {
        Self { item_id }
    }
}
impl std::error::Error for ItemMutatedError {}
impl std::fmt::Display for ItemMutatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} is not mutated", self.item_id)
    }
}
