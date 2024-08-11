use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemKindRemoveError {
    pub item_kind: &'static str,
}
impl ItemKindRemoveError {
    pub(crate) fn new(item_kind: &'static str) -> Self {
        Self { item_kind }
    }
}
impl std::error::Error for ItemKindRemoveError {}
impl std::fmt::Display for ItemKindRemoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} cannot be manually removed", self.item_kind)
    }
}
