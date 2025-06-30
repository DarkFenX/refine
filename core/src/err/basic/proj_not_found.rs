use crate::def::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("projection {projector_item_id}->{projectee_item_id} is already defined")]
pub struct ProjNotFoundError {
    pub projector_item_id: ItemId,
    pub projectee_item_id: ItemId,
}
