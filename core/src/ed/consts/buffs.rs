// Using buff developer descriptions as names here

use crate::ed::EBuffId;

pub(crate) const VELOCITY_PENALTY: EBuffId = EBuffId::new(3);
pub(crate) const WARP_PENALTY: EBuffId = EBuffId::new(4);
pub(crate) const DISALLOW_CLOAK: EBuffId = EBuffId::new(5);
pub(crate) const DISALLOW_DOCK_JUMP: EBuffId = EBuffId::new(6);
pub(crate) const PANIC_SHIELD_RESIST: EBuffId = EBuffId::new(8);
pub(crate) const PANIC_SHIELD_RECHARGE_TIME: EBuffId = EBuffId::new(9);
pub(crate) const STASIS_WEBIFICATION_BURST: EBuffId = EBuffId::new(27);
pub(crate) const WD_BURST_TURRET_MAX_RANGE: EBuffId = EBuffId::new(28);
pub(crate) const WD_BURST_TURRET_FALLOFF_RANGE: EBuffId = EBuffId::new(29);
pub(crate) const WD_BURST_TURRET_TRACKING: EBuffId = EBuffId::new(30);
pub(crate) const WD_BURST_MISSILE_VELOCITY: EBuffId = EBuffId::new(31);
pub(crate) const WD_BURST_MISSILE_DURATION: EBuffId = EBuffId::new(32);
pub(crate) const WD_BURST_MISSILE_EXPLOSION_VELOCITY: EBuffId = EBuffId::new(33);
pub(crate) const WD_BURST_MISSILE_EXPLOSION_RADIUS: EBuffId = EBuffId::new(34);
pub(crate) const DAMP_BURST_SCAN_RESOLUTION_PENALTY: EBuffId = EBuffId::new(35);
pub(crate) const DAMP_BURST_TARGETING_RANGE_PENALTY: EBuffId = EBuffId::new(36);
pub(crate) const SIGNATURE_RADIUS_PENALTY: EBuffId = EBuffId::new(37);
pub(crate) const PANIC_SCAN_RESOLUTION_PENALTY: EBuffId = EBuffId::new(55);
pub(crate) const PANIC_MASS_INCREASE: EBuffId = EBuffId::new(56);
pub(crate) const DISALLOW_TETHER: EBuffId = EBuffId::new(57);
pub(crate) const PANIC_DRONE_DMG_PENALTY: EBuffId = EBuffId::new(58);
pub(crate) const PANIC_DISALLOW_WEAPONS: EBuffId = EBuffId::new(59);
pub(crate) const PANIC_DISALLOW_ENTOSIS: EBuffId = EBuffId::new(61);
pub(crate) const REMOTE_REPAIR_IMPEDANCE: EBuffId = EBuffId::new(2201);
pub(crate) const SOV_SMOD_SHIELD_HITPOINT_BONUS: EBuffId = EBuffId::new(2433);
pub(crate) const SOV_SMOD_CAPACITOR_CAPACITY_BONUS: EBuffId = EBuffId::new(2434);
pub(crate) const SOV_SMOD_ARMOR_HITPOINT_BONUS: EBuffId = EBuffId::new(2435);
pub(crate) const SOV_SMOD_MODULE_OVERHEAT_BONUS: EBuffId = EBuffId::new(2436);
pub(crate) const SOV_SMOD_CAPACITOR_RECHARGE_BONUS: EBuffId = EBuffId::new(2437);
pub(crate) const SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS: EBuffId = EBuffId::new(2438);
pub(crate) const SOV_SMOD_SCAN_RESOLUTION_BONUS: EBuffId = EBuffId::new(2439);
pub(crate) const SOV_SMOD_WARP_SPEED_ADD: EBuffId = EBuffId::new(2440);
pub(crate) const SOV_SMOD_SHIELD_BOOSTER_BONUS: EBuffId = EBuffId::new(2441);
pub(crate) const SOV_SMOD_ARMOR_REPAIRER_BONUS: EBuffId = EBuffId::new(2442);
