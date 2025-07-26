use crate::{rd, ud::UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_key: UItemKey,
    pub(crate) effect_key: rd::REffectKey,
}
impl EffectSpec {
    pub(crate) fn new(item_key: UItemKey, effect_key: rd::REffectKey) -> Self {
        Self { item_key, effect_key }
    }
}
