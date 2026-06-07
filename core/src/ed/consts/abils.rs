use crate::ed::EAbilId;

impl EAbilId {
    pub(crate) const STASIS_WEB: Self = Self::from_i32(2);
    pub(crate) const MICRO_WARP_DRIVE: Self = Self::from_i32(4);
    pub(crate) const MICRO_JUMP_DRIVE: Self = Self::from_i32(5);
    pub(crate) const LAUNCH_BOMB: Self = Self::from_i32(7);
    pub(crate) const AFTERBURNER: Self = Self::from_i32(9);
    pub(crate) const WARP_DISRUPT: Self = Self::from_i32(10);
    pub(crate) const ENERGY_NEUT: Self = Self::from_i32(11);
    pub(crate) const ECM: Self = Self::from_i32(12);
    pub(crate) const EVASIVE_MANEUVERS: Self = Self::from_i32(13);
    pub(crate) const TACKLE: Self = Self::from_i32(16);
    pub(crate) const TORPEDO_SALVO_EM: Self = Self::from_i32(18);
    pub(crate) const TORPEDO_SALVO_THERM: Self = Self::from_i32(19);
    pub(crate) const TORPEDO_SALVO_KIN: Self = Self::from_i32(20);
    pub(crate) const TORPEDO_SALVO_EXP: Self = Self::from_i32(21);
    pub(crate) const PULSE_CANNON: Self = Self::from_i32(22);
    pub(crate) const BEAM_CANNON: Self = Self::from_i32(23);
    pub(crate) const BLASTER_CANNON_THERM: Self = Self::from_i32(24);
    pub(crate) const RAILGUN_THERM: Self = Self::from_i32(25);
    pub(crate) const AUTOCANNON: Self = Self::from_i32(26);
    pub(crate) const ARTILLERY: Self = Self::from_i32(27);
    pub(crate) const UMISSILE_SWARM_EM: Self = Self::from_i32(29);
    pub(crate) const UMISSILE_SWARM_THERM: Self = Self::from_i32(30);
    pub(crate) const UMISSILE_SWARM_KIN: Self = Self::from_i32(31);
    pub(crate) const UMISSILE_SWARM_EXP: Self = Self::from_i32(32);
    pub(crate) const HEAVY_ROCKET_SALVO_EM: Self = Self::from_i32(33);
    pub(crate) const HEAVY_ROCKET_SALVO_THERM: Self = Self::from_i32(34);
    pub(crate) const HEAVY_ROCKET_SALVO_KIN: Self = Self::from_i32(35);
    pub(crate) const HEAVY_ROCKET_SALVO_EXP: Self = Self::from_i32(36);
    pub(crate) const TRUE_SACRIFICE: Self = Self::from_i32(38);
    pub(crate) const BLASTER_CANNON_KIN: Self = Self::from_i32(44);
    pub(crate) const RAILGUN_KIN: Self = Self::from_i32(45);
}
