use crate::ud::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("{item_kind} {item_id} can't receive projections")]
pub struct ItemReceiveProjError {
    pub item_id: ItemId,
    pub item_kind: &'static str,
}
