use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemKindMatchError {
    pub item_id: SolItemId,
    pub expected_kind: &'static str,
    pub actual_kind: &'static str,
}
impl ItemKindMatchError {
    pub(crate) fn new(item_id: SolItemId, expected_kind: &'static str, actual_kind: &'static str) -> Self {
        Self {
            item_id,
            expected_kind,
            actual_kind,
        }
    }
}
impl std::error::Error for ItemKindMatchError {}
impl std::fmt::Display for ItemKindMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "item {} was requested as {}. but is {}",
            self.item_id, self.expected_kind, self.actual_kind
        )
    }
}
