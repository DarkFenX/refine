use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemKindRemoveError {
    pub item_id: SolItemId,
    pub item_kind: &'static str,
}
impl ItemKindRemoveError {
    pub(crate) fn new(item_id: SolItemId, item_kind: &'static str) -> Self {
        Self { item_id, item_kind }
    }
}
impl std::error::Error for ItemKindRemoveError {}
impl std::fmt::Display for ItemKindRemoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} cannot be removed", self.item_kind, self.item_id)
    }
}
