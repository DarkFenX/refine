use crate::sol::ItemId;

#[derive(Debug)]
pub struct ChargeFoundError {
    pub cont_item_id: ItemId,
}
impl std::error::Error for ChargeFoundError {}
impl std::fmt::Display for ChargeFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} does not have charge set", self.cont_item_id)
    }
}
