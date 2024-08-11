use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ChargeFoundError {
    pub cont_item_id: SolItemId,
}
impl ChargeFoundError {
    pub(crate) fn new(cont_item_id: SolItemId) -> Self {
        Self { cont_item_id }
    }
}
impl std::error::Error for ChargeFoundError {}
impl std::fmt::Display for ChargeFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item {} does not have charge set", self.cont_item_id)
    }
}
