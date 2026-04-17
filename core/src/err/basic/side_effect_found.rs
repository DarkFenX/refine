use crate::{api::EffectId, ud::ItemId};

#[derive(thiserror::Error, Debug)]
#[error("effect {effect_id} is not a side effect on item {item_id}")]
pub struct SideEffectFoundError {
    pub item_id: ItemId,
    pub effect_id: EffectId,
}
