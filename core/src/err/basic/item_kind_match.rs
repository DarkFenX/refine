use crate::sol::ItemId;

#[derive(Debug)]
pub struct ItemKindMatchError {
    pub item_id: ItemId,
    pub expected_kind: &'static str,
    pub actual_kind: &'static str,
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
