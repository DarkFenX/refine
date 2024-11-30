use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemNotMutatedError {
    pub item_id: SolItemId,
}
impl ItemNotMutatedError {
    pub(crate) fn new(item_id: SolItemId) -> Self {
        Self { item_id }
    }
}
impl std::error::Error for ItemNotMutatedError {}
impl std::fmt::Display for ItemNotMutatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} is mutated", self.item_id)
    }
}
