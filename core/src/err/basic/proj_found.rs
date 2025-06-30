use crate::def::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("projection {projector_item_id}->{projectee_item_id} not found")]
pub struct ProjFoundError {
    pub projector_item_id: ItemId,
    pub projectee_item_id: ItemId,
}
