use crate::ed::EAbilId;

impl EAbilId {
    pub(crate) const WEB: Self = Self::from_i32(2);
    pub(crate) const MWD: Self = Self::from_i32(4);
    pub(crate) const MJD: Self = Self::from_i32(5);
    pub(crate) const BOMB: Self = Self::from_i32(7);
    pub(crate) const AB: Self = Self::from_i32(9);
    pub(crate) const POINT: Self = Self::from_i32(10);
    pub(crate) const NEUT: Self = Self::from_i32(11);
    pub(crate) const ECM: Self = Self::from_i32(12);
    pub(crate) const EVASION: Self = Self::from_i32(13);
    pub(crate) const TACKLE: Self = Self::from_i32(16);
    pub(crate) const TORP_EM: Self = Self::from_i32(18);
    pub(crate) const TORP_THERM: Self = Self::from_i32(19);
    pub(crate) const TORP_KIN: Self = Self::from_i32(20);
    pub(crate) const TORP_EXP: Self = Self::from_i32(21);
    pub(crate) const ATK_PULSE: Self = Self::from_i32(22);
    pub(crate) const ATK_BEAM: Self = Self::from_i32(23);
    pub(crate) const ATK_BLASTER_THERM: Self = Self::from_i32(24);
    pub(crate) const ATK_RAIL_THERM: Self = Self::from_i32(25);
    pub(crate) const ATK_AUTOCANNON: Self = Self::from_i32(26);
    pub(crate) const ATK_ARTY: Self = Self::from_i32(27);
    pub(crate) const UMISSILE_EM: Self = Self::from_i32(29);
    pub(crate) const UMISSILE_THERM: Self = Self::from_i32(30);
    pub(crate) const UMISSILE_KIN: Self = Self::from_i32(31);
    pub(crate) const UMISSILE_EXP: Self = Self::from_i32(32);
    pub(crate) const ROCKET_EM: Self = Self::from_i32(33);
    pub(crate) const ROCKET_THERM: Self = Self::from_i32(34);
    pub(crate) const ROCKET_KIN: Self = Self::from_i32(35);
    pub(crate) const ROCKET_EXP: Self = Self::from_i32(36);
    pub(crate) const KAMIKAZE: Self = Self::from_i32(38);
    pub(crate) const ATK_BLASTER_KIN: Self = Self::from_i32(44);
    pub(crate) const ATK_RAIL_KIN: Self = Self::from_i32(45);
}
