use crate::{
    ad::{ABuffId, ACustomBuffId},
    ed::EBuffId,
};

impl ABuffId {
    pub(crate) const VELOCITY_PENALTY: Self = Self::from_eid(EBuffId::VELOCITY_PENALTY);
    pub(crate) const WARP_PENALTY: Self = Self::from_eid(EBuffId::WARP_PENALTY);
    pub(crate) const DISALLOW_CLOAK: Self = Self::from_eid(EBuffId::DISALLOW_CLOAK);
    pub(crate) const DISALLOW_DOCK_JUMP: Self = Self::from_eid(EBuffId::DISALLOW_DOCK_JUMP);
    pub(crate) const PANIC_SHIELD_RESIST: Self = Self::from_eid(EBuffId::PANIC_SHIELD_RESIST);
    pub(crate) const PANIC_SHIELD_RECHARGE_TIME: Self = Self::from_eid(EBuffId::PANIC_SHIELD_RECHARGE_TIME);
    pub(crate) const STASIS_WEBIFICATION_BURST: Self = Self::from_eid(EBuffId::STASIS_WEBIFICATION_BURST);
    pub(crate) const WD_BURST_TURRET_MAX_RANGE: Self = Self::from_eid(EBuffId::WD_BURST_TURRET_MAX_RANGE);
    pub(crate) const WD_BURST_TURRET_FALLOFF_RANGE: Self = Self::from_eid(EBuffId::WD_BURST_TURRET_FALLOFF_RANGE);
    pub(crate) const WD_BURST_TURRET_TRACKING: Self = Self::from_eid(EBuffId::WD_BURST_TURRET_TRACKING);
    pub(crate) const WD_BURST_MISSILE_VELOCITY: Self = Self::from_eid(EBuffId::WD_BURST_MISSILE_VELOCITY);
    pub(crate) const WD_BURST_MISSILE_DURATION: Self = Self::from_eid(EBuffId::WD_BURST_MISSILE_DURATION);
    pub(crate) const WD_BURST_MISSILE_EXPLOSION_VELOCITY: Self =
        Self::from_eid(EBuffId::WD_BURST_MISSILE_EXPLOSION_VELOCITY);
    pub(crate) const WD_BURST_MISSILE_EXPLOSION_RADIUS: Self =
        Self::from_eid(EBuffId::WD_BURST_MISSILE_EXPLOSION_RADIUS);
    pub(crate) const DAMP_BURST_SCAN_RESOLUTION_PENALTY: Self =
        Self::from_eid(EBuffId::DAMP_BURST_SCAN_RESOLUTION_PENALTY);
    pub(crate) const DAMP_BURST_TARGETING_RANGE_PENALTY: Self =
        Self::from_eid(EBuffId::DAMP_BURST_TARGETING_RANGE_PENALTY);
    pub(crate) const SIGNATURE_RADIUS_PENALTY: Self = Self::from_eid(EBuffId::SIGNATURE_RADIUS_PENALTY);
    pub(crate) const PANIC_SCAN_RESOLUTION_PENALTY: Self = Self::from_eid(EBuffId::PANIC_SCAN_RESOLUTION_PENALTY);
    pub(crate) const PANIC_MASS_INCREASE: Self = Self::from_eid(EBuffId::PANIC_MASS_INCREASE);
    pub(crate) const DISALLOW_TETHER: Self = Self::from_eid(EBuffId::DISALLOW_TETHER);
    pub(crate) const PANIC_DRONE_DMG_PENALTY: Self = Self::from_eid(EBuffId::PANIC_DRONE_DMG_PENALTY);
    pub(crate) const PANIC_DISALLOW_WEAPONS: Self = Self::from_eid(EBuffId::PANIC_DISALLOW_WEAPONS);
    pub(crate) const PANIC_DISALLOW_ENTOSIS: Self = Self::from_eid(EBuffId::PANIC_DISALLOW_ENTOSIS);
    pub(crate) const REMOTE_REPAIR_IMPEDANCE: Self = Self::from_eid(EBuffId::REMOTE_REPAIR_IMPEDANCE);
    pub(crate) const SOV_SMOD_SHIELD_HITPOINT_BONUS: Self = Self::from_eid(EBuffId::SOV_SMOD_SHIELD_HITPOINT_BONUS);
    pub(crate) const SOV_SMOD_CAPACITOR_CAPACITY_BONUS: Self =
        Self::from_eid(EBuffId::SOV_SMOD_CAPACITOR_CAPACITY_BONUS);
    pub(crate) const SOV_SMOD_ARMOR_HITPOINT_BONUS: Self = Self::from_eid(EBuffId::SOV_SMOD_ARMOR_HITPOINT_BONUS);
    pub(crate) const SOV_SMOD_MODULE_OVERHEAT_BONUS: Self = Self::from_eid(EBuffId::SOV_SMOD_MODULE_OVERHEAT_BONUS);
    pub(crate) const SOV_SMOD_CAPACITOR_RECHARGE_BONUS: Self =
        Self::from_eid(EBuffId::SOV_SMOD_CAPACITOR_RECHARGE_BONUS);
    pub(crate) const SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS: Self =
        Self::from_eid(EBuffId::SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS);
    pub(crate) const SOV_SMOD_SCAN_RESOLUTION_BONUS: Self = Self::from_eid(EBuffId::SOV_SMOD_SCAN_RESOLUTION_BONUS);
    pub(crate) const SOV_SMOD_WARP_SPEED_ADD: Self = Self::from_eid(EBuffId::SOV_SMOD_WARP_SPEED_ADD);
    pub(crate) const SOV_SMOD_SHIELD_BOOSTER_BONUS: Self = Self::from_eid(EBuffId::SOV_SMOD_SHIELD_BOOSTER_BONUS);
    pub(crate) const SOV_SMOD_ARMOR_REPAIRER_BONUS: Self = Self::from_eid(EBuffId::SOV_SMOD_ARMOR_REPAIRER_BONUS);
    // Library-specific buffs
    pub(crate) const DISALLOW_WARP_JUMP: Self = Self::Custom(ACustomBuffId::from_i32(1));
}
