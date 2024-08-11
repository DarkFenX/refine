use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemRemoveError {
    pub item_id: SolItemId,
    pub item_kind: &'static str,
}
impl ItemRemoveError {
    pub(crate) fn new(item_id: SolItemId, item_kind: &'static str) -> Self {
        Self { item_id, item_kind }
    }
}
impl std::error::Error for ItemRemoveError {}
impl std::fmt::Display for ItemRemoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} cannot be modified", self.item_kind, self.item_id)
    }
}
