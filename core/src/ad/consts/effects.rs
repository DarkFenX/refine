#![allow(dead_code)]

use crate::{ad::AEffectId, ed::consts::effects as ece};

pub(crate) const LO_POWER: AEffectId = AEffectId::Dogma(ece::LO_POWER);
pub(crate) const HI_POWER: AEffectId = AEffectId::Dogma(ece::HI_POWER);
pub(crate) const MED_POWER: AEffectId = AEffectId::Dogma(ece::MED_POWER);
pub(crate) const ONLINE: AEffectId = AEffectId::Dogma(ece::ONLINE);
pub(crate) const LAUNCHER_FITTED: AEffectId = AEffectId::Dogma(ece::LAUNCHER_FITTED);
pub(crate) const TURRET_FITTED: AEffectId = AEffectId::Dogma(ece::TURRET_FITTED);
pub(crate) const USE_MISSILES: AEffectId = AEffectId::Dogma(ece::USE_MISSILES);
pub(crate) const MISSILE_EM_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_EM_DMG_BONUS);
pub(crate) const MISSILE_EXPL_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_EXPL_DMG_BONUS);
pub(crate) const MISSILE_THERM_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_THERM_DMG_BONUS);
pub(crate) const MISSILE_KIN_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_KIN_DMG_BONUS);
pub(crate) const DRONE_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::DRONE_DMG_BONUS);
pub(crate) const SELF_ROF: AEffectId = AEffectId::Dogma(ece::SELF_ROF);
pub(crate) const RIG_SLOT: AEffectId = AEffectId::Dogma(ece::RIG_SLOT);
pub(crate) const SUBSYSTEM: AEffectId = AEffectId::Dogma(ece::SUBSYSTEM);
pub(crate) const HARDPOINT_MODIFIER_EFFECT: AEffectId = AEffectId::Dogma(ece::HARDPOINT_MODIFIER_EFFECT);
pub(crate) const SLOT_MODIFIER: AEffectId = AEffectId::Dogma(ece::SLOT_MODIFIER);
pub(crate) const WARP_DISRUPT_SPHERE: AEffectId = AEffectId::Dogma(ece::WARP_DISRUPT_SPHERE);
pub(crate) const ADAPTIVE_ARMOR_HARDENER: AEffectId = AEffectId::Dogma(ece::ADAPTIVE_ARMOR_HARDENER);
pub(crate) const FUELED_ARMOR_REPAIR: AEffectId = AEffectId::Dogma(ece::FUELED_ARMOR_REPAIR);
pub(crate) const STRUCTURE_WARP_SCRAMBLE_BLOCK_MWD_WITH_NPC: AEffectId =
    AEffectId::Dogma(ece::STRUCTURE_WARP_SCRAMBLE_BLOCK_MWD_WITH_NPC);
pub(crate) const SERVICE_SLOT: AEffectId = AEffectId::Dogma(ece::SERVICE_SLOT);
pub(crate) const REMOTE_WEBIFIER_FALLOFF: AEffectId = AEffectId::Dogma(ece::REMOTE_WEBIFIER_FALLOFF);
pub(crate) const FTR_ABIL_MISSILES: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_MISSILES);
pub(crate) const FTR_ABIL_NEUT: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_NEUT);
pub(crate) const FTR_ABIL_WEB: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_WEB);
pub(crate) const FTR_ABIL_POINT: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_POINT);
pub(crate) const FTR_ABIL_ECM: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_ECM);
pub(crate) const FTR_ABIL_EVASION: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_EVASION);
pub(crate) const FTR_ABIL_AB: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_AB);
pub(crate) const FTR_ABIL_MWD: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_MWD);
pub(crate) const FTR_ABIL_MJD: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_MJD);
pub(crate) const FTR_ABIL_TACKLE: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_TACKLE);
pub(crate) const FTR_ABIL_ATK_MISSILE: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_ATK_MISSILE);
pub(crate) const DOOMSDAY_AOE_WEB: AEffectId = AEffectId::Dogma(ece::DOOMSDAY_AOE_WEB);
pub(crate) const FTR_ABIL_BOMB: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_BOMB);
pub(crate) const FTR_ABIL_KAMIKAZE: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_KAMIKAZE);
pub(crate) const SHIP_MODULE_ARAR: AEffectId = AEffectId::Dogma(ece::SHIP_MODULE_ARAR);
pub(crate) const STRUCTURE_MODULE_EFFECT_STASIS_WEBIFIER: AEffectId =
    AEffectId::Dogma(ece::STRUCTURE_MODULE_EFFECT_STASIS_WEBIFIER);
pub(crate) const REMOTE_WEBIFIER_ENTITY: AEffectId = AEffectId::Dogma(ece::REMOTE_WEBIFIER_ENTITY);
pub(crate) const MOD_BONUS_MICROWARPDRIVE: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_MICROWARPDRIVE);
pub(crate) const MOD_BONUS_AFTERBURNER: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_AFTERBURNER);
pub(crate) const MOD_BONUS_WARFARE_LINK_ARMOR: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_ARMOR);
pub(crate) const MOD_BONUS_WARFARE_LINK_SHIELD: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_SHIELD);
pub(crate) const MOD_BONUS_WARFARE_LINK_SKIRMISH: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_SKIRMISH);
pub(crate) const MOD_BONUS_WARFARE_LINK_INFO: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_INFO);
pub(crate) const MOD_BONUS_WARFARE_LINK_MINING: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_MINING);
pub(crate) const MOD_TITAN_EFFECT_GENERATOR: AEffectId = AEffectId::Dogma(ece::MOD_TITAN_EFFECT_GENERATOR);
pub(crate) const SHIP_MOD_FOCUSED_WARP_SCRAMBLING_SCRIPT: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_FOCUSED_WARP_SCRAMBLING_SCRIPT);
pub(crate) const SHIP_MOD_FOCUSED_WARP_DISRUPTION_SCRIPT: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_FOCUSED_WARP_DISRUPTION_SCRIPT);
pub(crate) const AOE_BEACON_BIOLUMINESCENCE_CLOUD: AEffectId = AEffectId::Dogma(ece::AOE_BEACON_BIOLUMINESCENCE_CLOUD);
pub(crate) const AOE_BEACON_CAUSTIC_CLOUD: AEffectId = AEffectId::Dogma(ece::AOE_BEACON_CAUSTIC_CLOUD);
pub(crate) const AOE_BEACON_FILAMENT_CLOUD: AEffectId = AEffectId::Dogma(ece::AOE_BEACON_FILAMENT_CLOUD);
pub(crate) const WEATHER_CAUSTIC_TOXIN: AEffectId = AEffectId::Dogma(ece::WEATHER_CAUSTIC_TOXIN);
pub(crate) const WEATHER_DARKNESS: AEffectId = AEffectId::Dogma(ece::WEATHER_DARKNESS);
pub(crate) const WEATHER_ELECTRIC_STORM: AEffectId = AEffectId::Dogma(ece::WEATHER_ELECTRIC_STORM);
pub(crate) const WEATHER_INFERNAL: AEffectId = AEffectId::Dogma(ece::WEATHER_INFERNAL);
pub(crate) const WEATHER_XENON_GAS: AEffectId = AEffectId::Dogma(ece::WEATHER_XENON_GAS);
pub(crate) const DEBUFF_LANCE: AEffectId = AEffectId::Dogma(ece::DEBUFF_LANCE);

// Library-specific effects
pub(crate) const REE_CHAR_MISSILE_DMG: AEffectId = AEffectId::Custom(1);
pub(crate) const REE_AAR_PASTE_BOOST: AEffectId = AEffectId::Custom(2);
pub(crate) const REE_STASIS_WEB_PROBE: AEffectId = AEffectId::Custom(3);
