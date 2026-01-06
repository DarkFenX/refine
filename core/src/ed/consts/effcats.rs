use crate::ed::EEffectCatId;

impl EEffectCatId {
    pub(crate) const PASSIVE: Self = Self::from_i32(0);
    pub(crate) const ACTIVE: Self = Self::from_i32(1);
    pub(crate) const TARGET: Self = Self::from_i32(2);
    pub(crate) const AREA: Self = Self::from_i32(3);
    pub(crate) const ONLINE: Self = Self::from_i32(4);
    pub(crate) const OVERLOAD: Self = Self::from_i32(5);
    pub(crate) const DUNGEON: Self = Self::from_i32(6);
    pub(crate) const SYSTEM: Self = Self::from_i32(7);
}
