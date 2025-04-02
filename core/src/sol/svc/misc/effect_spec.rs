use crate::{ad, sol::ItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc) struct EffectSpec {
    pub(in crate::sol::svc) item_id: ItemId,
    pub(in crate::sol::svc) a_effect_id: ad::AEffectId,
}
