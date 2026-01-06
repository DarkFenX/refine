use crate::ed::EItemCatId;

impl EItemCatId {
    pub(crate) const CHARGE: Self = Self::from_i32(8);
    pub(crate) const DRONE: Self = Self::from_i32(18);
    pub(crate) const FIGHTER: Self = Self::from_i32(87);
    pub(crate) const IMPLANT: Self = Self::from_i32(20);
    pub(crate) const MODULE: Self = Self::from_i32(7);
    pub(crate) const SHIP: Self = Self::from_i32(6);
    pub(crate) const SKILL: Self = Self::from_i32(16);
    pub(crate) const SUBSYSTEM: Self = Self::from_i32(32);
    pub(crate) const STRUCTURE: Self = Self::from_i32(65);
    pub(crate) const STRUCTURE_MODULE: Self = Self::from_i32(66);
}
