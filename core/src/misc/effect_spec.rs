use crate::{rd::REffectKey, ud::UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_key: UItemKey,
    pub(crate) effect_key: REffectKey,
}
impl EffectSpec {
    pub(crate) fn new(item_key: UItemKey, effect_key: REffectKey) -> Self {
        Self { item_key, effect_key }
    }
}
