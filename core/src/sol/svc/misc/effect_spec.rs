use crate::{ad, sol::ItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_key: ItemKey,
    pub(crate) a_effect_id: ad::AEffectId,
}
impl EffectSpec {
    pub(crate) fn new(item_key: ItemKey, a_effect_id: ad::AEffectId) -> Self {
        Self { item_key, a_effect_id }
    }
}
