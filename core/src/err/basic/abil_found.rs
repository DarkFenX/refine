use crate::{api::AbilId, ud::ItemId};

#[derive(thiserror::Error, Debug)]
#[error("ability {ability_id} is not found on item {item_id}")]
pub struct AbilityFoundError {
    pub item_id: ItemId,
    pub ability_id: AbilId,
}
