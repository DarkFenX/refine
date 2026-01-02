use crate::{rd::REffectKey, ud::UItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_key: UItemId,
    pub(crate) effect_key: REffectKey,
}
impl EffectSpec {
    pub(crate) fn new(item_key: UItemId, effect_key: REffectKey) -> Self {
        Self { item_key, effect_key }
    }
}
