use crate::{ad::AItemCatId, ed::EItemCatId};

impl AItemCatId {
    pub(crate) const CHARGE: Self = Self::from_eid(EItemCatId::CHARGE);
    pub(crate) const DRONE: Self = Self::from_eid(EItemCatId::DRONE);
    pub(crate) const FIGHTER: Self = Self::from_eid(EItemCatId::FIGHTER);
    pub(crate) const IMPLANT: Self = Self::from_eid(EItemCatId::IMPLANT);
    pub(crate) const MODULE: Self = Self::from_eid(EItemCatId::MODULE);
    pub(crate) const SHIP: Self = Self::from_eid(EItemCatId::SHIP);
    pub(crate) const SKILL: Self = Self::from_eid(EItemCatId::SKILL);
    pub(crate) const SUBSYSTEM: Self = Self::from_eid(EItemCatId::SUBSYSTEM);
    pub(crate) const STRUCTURE: Self = Self::from_eid(EItemCatId::STRUCTURE);
    pub(crate) const STRUCTURE_MODULE: Self = Self::from_eid(EItemCatId::STRUCTURE_MODULE);
}
