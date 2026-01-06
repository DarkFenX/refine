use crate::{ad::AItemGrpId, ed::EItemGrpId};

impl AItemGrpId {
    pub(crate) const CHARACTER: Self = Self::from_eid(EItemGrpId::CHARACTER);
    pub(crate) const HAULER: Self = Self::from_eid(EItemGrpId::HAULER);
    pub(crate) const PROPULSION_MODULE: Self = Self::from_eid(EItemGrpId::PROPULSION_MODULE);
    pub(crate) const DEEP_SPACE_TRANSPORT: Self = Self::from_eid(EItemGrpId::DEEP_SPACE_TRANSPORT);
    pub(crate) const MINING_BARGE: Self = Self::from_eid(EItemGrpId::MINING_BARGE);
    pub(crate) const FREIGHTER: Self = Self::from_eid(EItemGrpId::FREIGHTER);
    pub(crate) const EXHUMER: Self = Self::from_eid(EItemGrpId::EXHUMER);
    pub(crate) const JUMP_FREIGHTER: Self = Self::from_eid(EItemGrpId::JUMP_FREIGHTER);
    pub(crate) const EFFECT_BEACON: Self = Self::from_eid(EItemGrpId::EFFECT_BEACON);
    pub(crate) const INDUSTRIAL_COMMAND_SHIP: Self = Self::from_eid(EItemGrpId::INDUSTRIAL_COMMAND_SHIP);
    pub(crate) const STRATEGIC_CRUISER: Self = Self::from_eid(EItemGrpId::STRATEGIC_CRUISER);
    pub(crate) const BLOCKADE_RUNNER: Self = Self::from_eid(EItemGrpId::BLOCKADE_RUNNER);
    pub(crate) const EXPEDITION_FRIGATE: Self = Self::from_eid(EItemGrpId::EXPEDITION_FRIGATE);
    pub(crate) const SHIP_MODIFIER: Self = Self::from_eid(EItemGrpId::SHIP_MODIFIER);
    pub(crate) const GUIDED_BOMB: Self = Self::from_eid(EItemGrpId::GUIDED_BOMB);
    pub(crate) const SOV_HUB_SYSTEM_EFFECT_GENERATOR_UPGRADES: Self =
        Self::from_eid(EItemGrpId::SOV_HUB_SYSTEM_EFFECT_GENERATOR_UPGRADES);
}
