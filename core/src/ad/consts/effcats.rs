use crate::{ad::AEffectCatId, ed::EEffectCatId};

impl AEffectCatId {
    pub(crate) const PASSIVE: Self = Self::from_eid(EEffectCatId::PASSIVE);
    pub(crate) const ACTIVE: Self = Self::from_eid(EEffectCatId::ACTIVE);
    pub(crate) const TARGET: Self = Self::from_eid(EEffectCatId::TARGET);
    pub(crate) const AREA: Self = Self::from_eid(EEffectCatId::AREA);
    pub(crate) const ONLINE: Self = Self::from_eid(EEffectCatId::ONLINE);
    pub(crate) const OVERLOAD: Self = Self::from_eid(EEffectCatId::OVERLOAD);
    pub(crate) const DUNGEON: Self = Self::from_eid(EEffectCatId::DUNGEON);
    pub(crate) const SYSTEM: Self = Self::from_eid(EEffectCatId::SYSTEM);
}
