use crate::sol::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {cont_item_id} does not have charge set")]
pub struct ChargeFoundError {
    pub cont_item_id: ItemId,
}
