use crate::ed::EItemGrpId;

impl EItemGrpId {
    pub(crate) const CHARACTER: Self = Self::from_i32(1);
    pub(crate) const HAULER: Self = Self::from_i32(28);
    pub(crate) const PROPULSION_MODULE: Self = Self::from_i32(46);
    pub(crate) const DEEP_SPACE_TRANSPORT: Self = Self::from_i32(380);
    pub(crate) const MINING_BARGE: Self = Self::from_i32(463);
    pub(crate) const FREIGHTER: Self = Self::from_i32(513);
    pub(crate) const EXHUMER: Self = Self::from_i32(543);
    pub(crate) const JUMP_FREIGHTER: Self = Self::from_i32(902);
    pub(crate) const EFFECT_BEACON: Self = Self::from_i32(920);
    pub(crate) const INDUSTRIAL_COMMAND_SHIP: Self = Self::from_i32(941);
    pub(crate) const STRATEGIC_CRUISER: Self = Self::from_i32(963);
    pub(crate) const BLOCKADE_RUNNER: Self = Self::from_i32(1202);
    pub(crate) const EXPEDITION_FRIGATE: Self = Self::from_i32(1283);
    pub(crate) const SHIP_MODIFIER: Self = Self::from_i32(1306);
    pub(crate) const GUIDED_BOMB: Self = Self::from_i32(1548);
    pub(crate) const SOV_HUB_SYSTEM_EFFECT_GENERATOR_UPGRADES: Self = Self::from_i32(4839);
}
