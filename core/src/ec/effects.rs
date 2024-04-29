#![allow(dead_code)]

use crate::defs::EEffectId;

// Slot-related
pub(crate) const HI_POWER: EEffectId = 12;
pub(crate) const LO_POWER: EEffectId = 11;
pub(crate) const MED_POWER: EEffectId = 13;
pub(crate) const RIG_SLOT: EEffectId = 2663;
pub(crate) const SUBSYSTEM: EEffectId = 3772;
// Buff-related
pub(crate) const MOD_BONUS_WARFARE_LINK_ARMOR: EEffectId = 6732;
pub(crate) const MOD_BONUS_WARFARE_LINK_INFO: EEffectId = 6735;
pub(crate) const MOD_BONUS_WARFARE_LINK_MINING: EEffectId = 6736;
pub(crate) const MOD_BONUS_WARFARE_LINK_SHIELD: EEffectId = 6733;
pub(crate) const MOD_BONUS_WARFARE_LINK_SKIRMISH: EEffectId = 6734;
pub(crate) const WEATHER_ELECTRIC_STORM: EEffectId = 7061;
pub(crate) const WEATHER_INFERNAL: EEffectId = 7062;
pub(crate) const WEATHER_CAUSTIC_TOXIN: EEffectId = 7059;
pub(crate) const WEATHER_XENON_GAS: EEffectId = 7063;
pub(crate) const WEATHER_DARKNESS: EEffectId = 7060;
pub(crate) const AOE_BEACON_BIOLUMINESCENCE_CLOUD: EEffectId = 7050;
pub(crate) const AOE_BEACON_CAUSTIC_CLOUD: EEffectId = 7050;
pub(crate) const AOE_BEACON_FILAMENT_CLOUD: EEffectId = 7058;
// Fighter-related
pub(crate) const FTR_ABIL_MISSILES: EEffectId = 6431;
pub(crate) const FTR_ABIL_NEUT: EEffectId = 6434;
pub(crate) const FTR_ABIL_WEB: EEffectId = 6435;
pub(crate) const FTR_ABIL_POINT: EEffectId = 6436;
pub(crate) const FTR_ABIL_ECM: EEffectId = 6437;
pub(crate) const FTR_ABIL_EVASION: EEffectId = 6439;
pub(crate) const FTR_ABIL_AB: EEffectId = 6440;
pub(crate) const FTR_ABIL_MWD: EEffectId = 6441;
pub(crate) const FTR_ABIL_MJD: EEffectId = 6442;
pub(crate) const FTR_ABIL_TACKLE: EEffectId = 6464;
pub(crate) const FTR_ABIL_ATK_MISSILE: EEffectId = 6465;
pub(crate) const FTR_ABIL_BOMB: EEffectId = 6485;
pub(crate) const FTR_ABIL_KAMIKAZE: EEffectId = 6554;
// Effects which need customizations
pub(crate) const ONLINE: EEffectId = 16;
pub(crate) const DRONE_DMG_BONUS: EEffectId = 1730;
pub(crate) const SELF_ROF: EEffectId = 1851;
pub(crate) const MISSILE_EM_DMG_BONUS: EEffectId = 660;
pub(crate) const MISSILE_THERM_DMG_BONUS: EEffectId = 662;
pub(crate) const MISSILE_KIN_DMG_BONUS: EEffectId = 668;
pub(crate) const MISSILE_EXPL_DMG_BONUS: EEffectId = 661;
pub(crate) const HARDPOINT_MODIFIER_EFFECT: EEffectId = 3773;
pub(crate) const SLOT_MODIFIER: EEffectId = 3774;
pub(crate) const MOD_BONUS_AFTERBURNER: EEffectId = 6731;
pub(crate) const MOD_BONUS_MICROWARPDRIVE: EEffectId = 6730;
pub(crate) const DOOMSDAY_AOE_WEB: EEffectId = 6476;
// Don't need customization by themselves, but are one of criteria for customizations
pub(crate) const FUELED_ARMOR_REPAIR: EEffectId = 5275;
pub(crate) const SHIP_MODULE_ARAR: EEffectId = 6651;
// Library-specific effects
pub(crate) const REE_CHAR_MISSILE_DMG: EEffectId = -1;
pub(crate) const REE_AAR_PASTE_BOOST: EEffectId = -2;
