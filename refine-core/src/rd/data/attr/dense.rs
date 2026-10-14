use crate::ad::AAttrId;

// TODO: improve the set but running large set of realistic scenarios (can share with PGO profile)
/// Attributes which are used the most often are listed here, their item values are stored densely.
pub(crate) const DENSE_ATTR_AIDS: [AAttrId; 43] = [
    // Resource use - validation
    AAttrId::POWER,
    AAttrId::CPU,
    // Cap use on most items - validation (max cap use) & stats (cap balance/sim)
    AAttrId::CAPACITOR_NEED,
    // Validation
    AAttrId::ACTIVATION_BLOCKED,
    // Slots - validation & stats
    AAttrId::HI_SLOTS,
    AAttrId::MED_SLOTS,
    AAttrId::LOW_SLOTS,
    AAttrId::TURRET_SLOTS_LEFT,
    AAttrId::LAUNCHER_SLOTS_LEFT,
    // Resource use - validation
    AAttrId::DRONE_BANDWIDTH,
    // Drone slots - validation & stats
    AAttrId::MAX_ACTIVE_DRONES,
    // Max group restriction - validation
    AAttrId::MAX_GROUP_FITTED,
    AAttrId::MAX_GROUP_ONLINE,
    AAttrId::MAX_GROUP_ACTIVE,
    // Rigs - validation & stats
    AAttrId::UPGRADE_SLOTS_LEFT,
    AAttrId::UPGRADE_CAPACITY,
    // Resource output - validation & stats
    AAttrId::POWER_OUTPUT,
    AAttrId::CPU_OUTPUT,
    AAttrId::DRONE_CAPACITY,
    // Fighter-related - used in validation and stats
    AAttrId::FTR_TUBES,
    AAttrId::FTR_SUPPORT_SLOTS,
    AAttrId::FTR_LIGHT_SLOTS,
    AAttrId::FTR_HEAVY_SLOTS,
    AAttrId::FTR_ST_SUPPORT_SLOTS,
    AAttrId::FTR_ST_LIGHT_SLOTS,
    AAttrId::FTR_ST_HEAVY_SLOTS,
    // Cap amount - validation & stats
    AAttrId::CAPACITOR_CAPACITY,
    // Other slots - validation & stats
    AAttrId::SERVICE_SLOTS,
    AAttrId::MAX_SUBSYSTEMS,
    // General
    AAttrId::SKILL_LEVEL,
    // Cycle-related
    AAttrId::SPEED,
    AAttrId::RELOAD_TIME,
    // Damage-related
    AAttrId::DMG_MULT,
    AAttrId::DMG_MULT_BONUS,
    AAttrId::EM_DMG,
    AAttrId::KIN_DMG,
    AAttrId::THERM_DMG,
    AAttrId::EXPL_DMG,
    // Damage-application-related
    AAttrId::MAX_RANGE,
    AAttrId::FALLOFF,
    AAttrId::TRACKING_SPEED,
    AAttrId::OPTIMAL_SIG_RADIUS,
    AAttrId::SIG_RADIUS,
];
/// Attributes with runtime IDs below this value are stored densely.
pub(crate) const DENSE_ATTR_COUNT: usize = DENSE_ATTR_AIDS.len();
