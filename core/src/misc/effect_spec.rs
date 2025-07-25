use crate::{ad, ud::UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_key: UItemKey,
    pub(crate) a_effect_id: ad::AEffectId,
}
impl EffectSpec {
    pub(crate) fn new(item_key: UItemKey, a_effect_id: ad::AEffectId) -> Self {
        Self { item_key, a_effect_id }
    }
}
