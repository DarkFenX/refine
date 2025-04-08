use crate::{ad, sol::ItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc) struct EffectSpec {
    pub(in crate::sol::svc) item_key: ItemKey,
    pub(in crate::sol::svc) a_effect_id: ad::AEffectId,
}
