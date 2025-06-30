use crate::def::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} was requested as {expected_kind}. but is {actual_kind}")]
pub struct ItemKindMatchError {
    pub item_id: ItemId,
    pub expected_kind: &'static str,
    pub actual_kind: &'static str,
}
