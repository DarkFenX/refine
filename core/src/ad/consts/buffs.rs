use crate::{
    ad::{ABuffId, ACustomBuffId},
    ec::buffs as ecb,
};

pub(crate) const VELOCITY_PENALTY: ABuffId = ecb::VELOCITY_PENALTY.into();
pub(crate) const WARP_PENALTY: ABuffId = ecb::WARP_PENALTY.into();
pub(crate) const DISALLOW_CLOAK: ABuffId = ecb::DISALLOW_CLOAK.into();
pub(crate) const DISALLOW_DOCK_JUMP: ABuffId = ecb::DISALLOW_DOCK_JUMP.into();
pub(crate) const PANIC_SHIELD_RESIST: ABuffId = ecb::PANIC_SHIELD_RESIST.into();
pub(crate) const PANIC_SHIELD_RECHARGE_TIME: ABuffId = ecb::PANIC_SHIELD_RECHARGE_TIME.into();
pub(crate) const STASIS_WEBIFICATION_BURST: ABuffId = ecb::STASIS_WEBIFICATION_BURST.into();
pub(crate) const WD_BURST_TURRET_MAX_RANGE: ABuffId = ecb::WD_BURST_TURRET_MAX_RANGE.into();
pub(crate) const WD_BURST_TURRET_FALLOFF_RANGE: ABuffId = ecb::WD_BURST_TURRET_FALLOFF_RANGE.into();
pub(crate) const WD_BURST_TURRET_TRACKING: ABuffId = ecb::WD_BURST_TURRET_TRACKING.into();
pub(crate) const WD_BURST_MISSILE_VELOCITY: ABuffId = ecb::WD_BURST_MISSILE_VELOCITY.into();
pub(crate) const WD_BURST_MISSILE_DURATION: ABuffId = ecb::WD_BURST_MISSILE_DURATION.into();
pub(crate) const WD_BURST_MISSILE_EXPLOSION_VELOCITY: ABuffId = ecb::WD_BURST_MISSILE_EXPLOSION_VELOCITY.into();
pub(crate) const WD_BURST_MISSILE_EXPLOSION_RADIUS: ABuffId = ecb::WD_BURST_MISSILE_EXPLOSION_RADIUS.into();
pub(crate) const DAMP_BURST_SCAN_RESOLUTION_PENALTY: ABuffId = ecb::DAMP_BURST_SCAN_RESOLUTION_PENALTY.into();
pub(crate) const DAMP_BURST_TARGETING_RANGE_PENALTY: ABuffId = ecb::DAMP_BURST_TARGETING_RANGE_PENALTY.into();
pub(crate) const SIGNATURE_RADIUS_PENALTY: ABuffId = ecb::SIGNATURE_RADIUS_PENALTY.into();
pub(crate) const PANIC_SCAN_RESOLUTION_PENALTY: ABuffId = ecb::PANIC_SCAN_RESOLUTION_PENALTY.into();
pub(crate) const PANIC_MASS_INCREASE: ABuffId = ecb::PANIC_MASS_INCREASE.into();
pub(crate) const DISALLOW_TETHER: ABuffId = ecb::DISALLOW_TETHER.into();
pub(crate) const PANIC_DRONE_DMG_PENALTY: ABuffId = ecb::PANIC_DRONE_DMG_PENALTY.into();
pub(crate) const PANIC_DISALLOW_WEAPONS: ABuffId = ecb::PANIC_DISALLOW_WEAPONS.into();
pub(crate) const PANIC_DISALLOW_ENTOSIS: ABuffId = ecb::PANIC_DISALLOW_ENTOSIS.into();
pub(crate) const REMOTE_REPAIR_IMPEDANCE: ABuffId = ecb::REMOTE_REPAIR_IMPEDANCE.into();
pub(crate) const SOV_SMOD_SHIELD_HITPOINT_BONUS: ABuffId = ecb::SOV_SMOD_SHIELD_HITPOINT_BONUS.into();
pub(crate) const SOV_SMOD_CAPACITOR_CAPACITY_BONUS: ABuffId = ecb::SOV_SMOD_CAPACITOR_CAPACITY_BONUS.into();
pub(crate) const SOV_SMOD_ARMOR_HITPOINT_BONUS: ABuffId = ecb::SOV_SMOD_ARMOR_HITPOINT_BONUS.into();
pub(crate) const SOV_SMOD_MODULE_OVERHEAT_BONUS: ABuffId = ecb::SOV_SMOD_MODULE_OVERHEAT_BONUS.into();
pub(crate) const SOV_SMOD_CAPACITOR_RECHARGE_BONUS: ABuffId = ecb::SOV_SMOD_CAPACITOR_RECHARGE_BONUS.into();
pub(crate) const SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS: ABuffId =
    ecb::SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS.into();
pub(crate) const SOV_SMOD_SCAN_RESOLUTION_BONUS: ABuffId = ecb::SOV_SMOD_SCAN_RESOLUTION_BONUS.into();
pub(crate) const SOV_SMOD_WARP_SPEED_ADD: ABuffId = ecb::SOV_SMOD_WARP_SPEED_ADD.into();
pub(crate) const SOV_SMOD_SHIELD_BOOSTER_BONUS: ABuffId = ecb::SOV_SMOD_SHIELD_BOOSTER_BONUS.into();
pub(crate) const SOV_SMOD_ARMOR_REPAIRER_BONUS: ABuffId = ecb::SOV_SMOD_ARMOR_REPAIRER_BONUS.into();

// Library-specific buffs
pub(crate) const DISALLOW_WARP_JUMP: ABuffId = ABuffId::Custom(ACustomBuffId::new(1));
