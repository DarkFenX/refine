use crate::ed::EBuffId;

impl EBuffId {
    // Using buff developer descriptions as names here
    pub(crate) const VELOCITY_PENALTY: Self = Self::from_i32(3);
    pub(crate) const WARP_PENALTY: Self = Self::from_i32(4);
    pub(crate) const DISALLOW_CLOAK: Self = Self::from_i32(5);
    pub(crate) const DISALLOW_DOCK_JUMP: Self = Self::from_i32(6);
    pub(crate) const PANIC_SHIELD_RESIST: Self = Self::from_i32(8);
    pub(crate) const PANIC_SHIELD_RECHARGE_TIME: Self = Self::from_i32(9);
    pub(crate) const STASIS_WEBIFICATION_BURST: Self = Self::from_i32(27);
    pub(crate) const WD_BURST_TURRET_MAX_RANGE: Self = Self::from_i32(28);
    pub(crate) const WD_BURST_TURRET_FALLOFF_RANGE: Self = Self::from_i32(29);
    pub(crate) const WD_BURST_TURRET_TRACKING: Self = Self::from_i32(30);
    pub(crate) const WD_BURST_MISSILE_VELOCITY: Self = Self::from_i32(31);
    pub(crate) const WD_BURST_MISSILE_DURATION: Self = Self::from_i32(32);
    pub(crate) const WD_BURST_MISSILE_EXPLOSION_VELOCITY: Self = Self::from_i32(33);
    pub(crate) const WD_BURST_MISSILE_EXPLOSION_RADIUS: Self = Self::from_i32(34);
    pub(crate) const DAMP_BURST_SCAN_RESOLUTION_PENALTY: Self = Self::from_i32(35);
    pub(crate) const DAMP_BURST_TARGETING_RANGE_PENALTY: Self = Self::from_i32(36);
    pub(crate) const SIGNATURE_RADIUS_PENALTY: Self = Self::from_i32(37);
    pub(crate) const PANIC_SCAN_RESOLUTION_PENALTY: Self = Self::from_i32(55);
    pub(crate) const PANIC_MASS_INCREASE: Self = Self::from_i32(56);
    pub(crate) const DISALLOW_TETHER: Self = Self::from_i32(57);
    pub(crate) const PANIC_DRONE_DMG_PENALTY: Self = Self::from_i32(58);
    pub(crate) const PANIC_DISALLOW_WEAPONS: Self = Self::from_i32(59);
    pub(crate) const PANIC_DISALLOW_ENTOSIS: Self = Self::from_i32(61);
    pub(crate) const REMOTE_REPAIR_IMPEDANCE: Self = Self::from_i32(2201);
    pub(crate) const SOV_SMOD_SHIELD_HITPOINT_BONUS: Self = Self::from_i32(2433);
    pub(crate) const SOV_SMOD_CAPACITOR_CAPACITY_BONUS: Self = Self::from_i32(2434);
    pub(crate) const SOV_SMOD_ARMOR_HITPOINT_BONUS: Self = Self::from_i32(2435);
    pub(crate) const SOV_SMOD_MODULE_OVERHEAT_BONUS: Self = Self::from_i32(2436);
    pub(crate) const SOV_SMOD_CAPACITOR_RECHARGE_BONUS: Self = Self::from_i32(2437);
    pub(crate) const SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS: Self = Self::from_i32(2438);
    pub(crate) const SOV_SMOD_SCAN_RESOLUTION_BONUS: Self = Self::from_i32(2439);
    pub(crate) const SOV_SMOD_WARP_SPEED_ADD: Self = Self::from_i32(2440);
    pub(crate) const SOV_SMOD_SHIELD_BOOSTER_BONUS: Self = Self::from_i32(2441);
    pub(crate) const SOV_SMOD_ARMOR_REPAIRER_BONUS: Self = Self::from_i32(2442);
}
