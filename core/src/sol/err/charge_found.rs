use crate::sol::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {cont_item_id} does not have charge set")]
pub(in crate::sol) struct ChargeFoundError {
    pub(in crate::sol) cont_item_id: ItemId,
}
