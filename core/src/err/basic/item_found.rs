use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemFoundError {
    pub item_id: SolItemId,
}
impl ItemFoundError {
    pub(crate) fn new(item_id: SolItemId) -> Self {
        Self { item_id }
    }
}
impl std::error::Error for ItemFoundError {}
impl std::fmt::Display for ItemFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} not found", self.item_id)
    }
}
