use crate::{ad, uad::UadItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_key: UadItemKey,
    pub(crate) a_effect_id: ad::AEffectId,
}
impl EffectSpec {
    pub(crate) fn new(item_key: UadItemKey, a_effect_id: ad::AEffectId) -> Self {
        Self { item_key, a_effect_id }
    }
}
