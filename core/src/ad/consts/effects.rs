// TODO: after everything is implemented, remove this and see what needs to be cleaned up
#![allow(dead_code)]

use crate::{ad::AEffectId, ed::consts::effects as ece};

pub(crate) const SHIELD_BOOSTING: AEffectId = AEffectId::Dogma(ece::SHIELD_BOOSTING);
pub(crate) const MISSILE_LAUNCHING: AEffectId = AEffectId::Dogma(ece::MISSILE_LAUNCHING);
pub(crate) const TGT_ATTACK: AEffectId = AEffectId::Dogma(ece::TGT_ATTACK);
pub(crate) const LO_POWER: AEffectId = AEffectId::Dogma(ece::LO_POWER);
pub(crate) const HI_POWER: AEffectId = AEffectId::Dogma(ece::HI_POWER);
pub(crate) const MED_POWER: AEffectId = AEffectId::Dogma(ece::MED_POWER);
pub(crate) const ONLINE: AEffectId = AEffectId::Dogma(ece::ONLINE);
pub(crate) const STRUCTURE_REPAIR: AEffectId = AEffectId::Dogma(ece::STRUCTURE_REPAIR);
pub(crate) const ARMOR_REPAIR: AEffectId = AEffectId::Dogma(ece::ARMOR_REPAIR);
pub(crate) const PROJECTILE_FIRED: AEffectId = AEffectId::Dogma(ece::PROJECTILE_FIRED);
pub(crate) const EMP_WAVE: AEffectId = AEffectId::Dogma(ece::EMP_WAVE);
pub(crate) const LAUNCHER_FITTED: AEffectId = AEffectId::Dogma(ece::LAUNCHER_FITTED);
pub(crate) const TURRET_FITTED: AEffectId = AEffectId::Dogma(ece::TURRET_FITTED);
pub(crate) const MINING_LASER: AEffectId = AEffectId::Dogma(ece::MINING_LASER);
pub(crate) const USE_MISSILES: AEffectId = AEffectId::Dogma(ece::USE_MISSILES);
pub(crate) const DEFENDER_MISSILE_LAUNCHING: AEffectId = AEffectId::Dogma(ece::DEFENDER_MISSILE_LAUNCHING);
pub(crate) const FOF_MISSILE_LAUNCHING: AEffectId = AEffectId::Dogma(ece::FOF_MISSILE_LAUNCHING);
pub(crate) const MISSILE_EM_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_EM_DMG_BONUS);
pub(crate) const MISSILE_EXPL_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_EXPL_DMG_BONUS);
pub(crate) const MISSILE_THERM_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_THERM_DMG_BONUS);
pub(crate) const MISSILE_KIN_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::MISSILE_KIN_DMG_BONUS);
pub(crate) const CLOAKING_TARGETING_DELAY_BONUS: AEffectId = AEffectId::Dogma(ece::CLOAKING_TARGETING_DELAY_BONUS);
pub(crate) const DRONE_DMG_BONUS: AEffectId = AEffectId::Dogma(ece::DRONE_DMG_BONUS);
pub(crate) const SELF_ROF: AEffectId = AEffectId::Dogma(ece::SELF_ROF);
pub(crate) const RIG_SLOT: AEffectId = AEffectId::Dogma(ece::RIG_SLOT);
pub(crate) const BOMB_LAUNCHING: AEffectId = AEffectId::Dogma(ece::BOMB_LAUNCHING);
pub(crate) const HARDPOINT_MODIFIER_EFFECT: AEffectId = AEffectId::Dogma(ece::HARDPOINT_MODIFIER_EFFECT);
pub(crate) const SLOT_MODIFIER: AEffectId = AEffectId::Dogma(ece::SLOT_MODIFIER);
pub(crate) const WARP_DISRUPT_SPHERE: AEffectId = AEffectId::Dogma(ece::WARP_DISRUPT_SPHERE);
pub(crate) const MICRO_JUMP_DRIVE: AEffectId = AEffectId::Dogma(ece::MICRO_JUMP_DRIVE);
pub(crate) const ADAPTIVE_ARMOR_HARDENER: AEffectId = AEffectId::Dogma(ece::ADAPTIVE_ARMOR_HARDENER);
pub(crate) const FUELED_SHIELD_BOOSTING: AEffectId = AEffectId::Dogma(ece::FUELED_SHIELD_BOOSTING);
pub(crate) const FUELED_ARMOR_REPAIR: AEffectId = AEffectId::Dogma(ece::FUELED_ARMOR_REPAIR);
pub(crate) const SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER);
pub(crate) const SHIP_MOD_REMOTE_HULL_REPAIRER: AEffectId = AEffectId::Dogma(ece::SHIP_MOD_REMOTE_HULL_REPAIRER);
pub(crate) const SHIP_MOD_REMOTE_SHIELD_BOOSTER: AEffectId = AEffectId::Dogma(ece::SHIP_MOD_REMOTE_SHIELD_BOOSTER);
pub(crate) const ENERGY_NEUT_FALLOFF: AEffectId = AEffectId::Dogma(ece::ENERGY_NEUT_FALLOFF);
pub(crate) const SHIP_MOD_REMOTE_ARMOR_REPAIRER: AEffectId = AEffectId::Dogma(ece::SHIP_MOD_REMOTE_ARMOR_REPAIRER);
pub(crate) const MICRO_JUMP_PORTAL_DRIVE: AEffectId = AEffectId::Dogma(ece::MICRO_JUMP_PORTAL_DRIVE);
pub(crate) const STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC: AEffectId =
    AEffectId::Dogma(ece::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC);
pub(crate) const SERVICE_SLOT: AEffectId = AEffectId::Dogma(ece::SERVICE_SLOT);
pub(crate) const REMOTE_SENSOR_DAMP_FALLOFF: AEffectId = AEffectId::Dogma(ece::REMOTE_SENSOR_DAMP_FALLOFF);
pub(crate) const SHIP_MOD_GUIDANCE_DISRUPTOR: AEffectId = AEffectId::Dogma(ece::SHIP_MOD_GUIDANCE_DISRUPTOR);
pub(crate) const SHIP_MOD_TRACKING_DISRUPTOR: AEffectId = AEffectId::Dogma(ece::SHIP_MOD_TRACKING_DISRUPTOR);
pub(crate) const REMOTE_WEBIFIER_FALLOFF: AEffectId = AEffectId::Dogma(ece::REMOTE_WEBIFIER_FALLOFF);
pub(crate) const REMOTE_SENSOR_BOOST_FALLOFF: AEffectId = AEffectId::Dogma(ece::REMOTE_SENSOR_BOOST_FALLOFF);
pub(crate) const FTR_ABIL_MISSILES: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_MISSILES);
pub(crate) const FTR_ABIL_NEUT: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_NEUT);
pub(crate) const FTR_ABIL_WEB: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_WEB);
pub(crate) const FTR_ABIL_POINT: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_POINT);
pub(crate) const FTR_ABIL_ECM: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_ECM);
pub(crate) const FTR_ABIL_EVASION: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_EVASION);
pub(crate) const FTR_ABIL_AB: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_AB);
pub(crate) const FTR_ABIL_MWD: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_MWD);
pub(crate) const FTR_ABIL_MJD: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_MJD);
pub(crate) const POINT_DEFENSE: AEffectId = AEffectId::Dogma(ece::POINT_DEFENSE);
pub(crate) const FTR_ABIL_TACKLE: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_TACKLE);
pub(crate) const FTR_ABIL_ATK_MISSILE: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_ATK_MISSILE);
pub(crate) const REMOTE_ECM_FALLOFF: AEffectId = AEffectId::Dogma(ece::REMOTE_ECM_FALLOFF);
pub(crate) const DOOMSDAY_AOE_WEB: AEffectId = AEffectId::Dogma(ece::DOOMSDAY_AOE_WEB);
pub(crate) const DOOMSDAY_AOE_TRACK: AEffectId = AEffectId::Dogma(ece::DOOMSDAY_AOE_TRACK);
pub(crate) const DOOMSDAY_AOE_DAMP: AEffectId = AEffectId::Dogma(ece::DOOMSDAY_AOE_DAMP);
pub(crate) const FTR_ABIL_BOMB: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_BOMB);
pub(crate) const DOOMSDAY_AOE_ECM: AEffectId = AEffectId::Dogma(ece::DOOMSDAY_AOE_ECM);
pub(crate) const FTR_ABIL_KAMIKAZE: AEffectId = AEffectId::Dogma(ece::FTR_ABIL_KAMIKAZE);
pub(crate) const SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER);
pub(crate) const SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER);
pub(crate) const STRUCT_MOD_EFFECT_STASIS_WEBIFIER: AEffectId =
    AEffectId::Dogma(ece::STRUCT_MOD_EFFECT_STASIS_WEBIFIER);
pub(crate) const STRUCT_MOD_EFFECT_REMOTE_SENSOR_DAMPENER: AEffectId =
    AEffectId::Dogma(ece::STRUCT_MOD_EFFECT_REMOTE_SENSOR_DAMPENER);
pub(crate) const STRUCT_MOD_EFFECT_ECM: AEffectId = AEffectId::Dogma(ece::STRUCT_MOD_EFFECT_ECM);
pub(crate) const STRUCT_MOD_EFFECT_WEAPON_DISRUPTION: AEffectId =
    AEffectId::Dogma(ece::STRUCT_MOD_EFFECT_WEAPON_DISRUPTION);
pub(crate) const NPC_ENTITY_REMOTE_ARMOR_REPAIRER: AEffectId = AEffectId::Dogma(ece::NPC_ENTITY_REMOTE_ARMOR_REPAIRER);
pub(crate) const NPC_ENTITY_REMOTE_SHIELD_BOOSTER: AEffectId = AEffectId::Dogma(ece::NPC_ENTITY_REMOTE_SHIELD_BOOSTER);
pub(crate) const NPC_ENTITY_REMOTE_HULL_REPAIRER: AEffectId = AEffectId::Dogma(ece::NPC_ENTITY_REMOTE_HULL_REPAIRER);
pub(crate) const REMOTE_SENSOR_DAMP_ENTITY: AEffectId = AEffectId::Dogma(ece::REMOTE_SENSOR_DAMP_ENTITY);
pub(crate) const NPC_ENTITY_WEAPON_DISRUPTOR: AEffectId = AEffectId::Dogma(ece::NPC_ENTITY_WEAPON_DISRUPTOR);
pub(crate) const REMOTE_WEBIFIER_ENTITY: AEffectId = AEffectId::Dogma(ece::REMOTE_WEBIFIER_ENTITY);
pub(crate) const ENTITY_ECM_FALLOFF: AEffectId = AEffectId::Dogma(ece::ENTITY_ECM_FALLOFF);
pub(crate) const ECM_BURST_JAMMER: AEffectId = AEffectId::Dogma(ece::ECM_BURST_JAMMER);
pub(crate) const MOD_BONUS_MICROWARPDRIVE: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_MICROWARPDRIVE);
pub(crate) const MOD_BONUS_AFTERBURNER: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_AFTERBURNER);
pub(crate) const MOD_BONUS_WARFARE_LINK_ARMOR: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_ARMOR);
pub(crate) const MOD_BONUS_WARFARE_LINK_SHIELD: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_SHIELD);
pub(crate) const MOD_BONUS_WARFARE_LINK_SKIRMISH: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_SKIRMISH);
pub(crate) const MOD_BONUS_WARFARE_LINK_INFO: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_INFO);
pub(crate) const MOD_BONUS_WARFARE_LINK_MINING: AEffectId = AEffectId::Dogma(ece::MOD_BONUS_WARFARE_LINK_MINING);
pub(crate) const MOD_TITAN_EFFECT_GENERATOR: AEffectId = AEffectId::Dogma(ece::MOD_TITAN_EFFECT_GENERATOR);
pub(crate) const SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT);
pub(crate) const SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT);
pub(crate) const TGT_DISINTEGRATOR_ATTACK: AEffectId = AEffectId::Dogma(ece::TGT_DISINTEGRATOR_ATTACK);
pub(crate) const AOE_BEACON_BIOLUMINESCENCE_CLOUD: AEffectId = AEffectId::Dogma(ece::AOE_BEACON_BIOLUMINESCENCE_CLOUD);
pub(crate) const AOE_BEACON_CAUSTIC_CLOUD: AEffectId = AEffectId::Dogma(ece::AOE_BEACON_CAUSTIC_CLOUD);
pub(crate) const AOE_BEACON_PULSE_01: AEffectId = AEffectId::Dogma(ece::AOE_BEACON_PULSE_01);
pub(crate) const AOE_BEACON_FILAMENT_CLOUD: AEffectId = AEffectId::Dogma(ece::AOE_BEACON_FILAMENT_CLOUD);
pub(crate) const WEATHER_CAUSTIC_TOXIN: AEffectId = AEffectId::Dogma(ece::WEATHER_CAUSTIC_TOXIN);
pub(crate) const WEATHER_DARKNESS: AEffectId = AEffectId::Dogma(ece::WEATHER_DARKNESS);
pub(crate) const WEATHER_ELECTRIC_STORM: AEffectId = AEffectId::Dogma(ece::WEATHER_ELECTRIC_STORM);
pub(crate) const WEATHER_INFERNAL: AEffectId = AEffectId::Dogma(ece::WEATHER_INFERNAL);
pub(crate) const WEATHER_XENON_GAS: AEffectId = AEffectId::Dogma(ece::WEATHER_XENON_GAS);
pub(crate) const SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER: AEffectId =
    AEffectId::Dogma(ece::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER);
pub(crate) const CHAIN_LIGHTNING: AEffectId = AEffectId::Dogma(ece::CHAIN_LIGHTNING);
pub(crate) const DEBUFF_LANCE: AEffectId = AEffectId::Dogma(ece::DEBUFF_LANCE);
pub(crate) const MICRO_JUMP_PORTAL_DRIVE_CAPITAL: AEffectId = AEffectId::Dogma(ece::MICRO_JUMP_PORTAL_DRIVE_CAPITAL);
pub(crate) const DOT_MISSILE_LAUNCHING: AEffectId = AEffectId::Dogma(ece::DOT_MISSILE_LAUNCHING);

// Library-specific effects
pub(crate) const CHAR_MISSILE_DMG: AEffectId = AEffectId::Custom(1);
pub(crate) const AAR_PASTE_BOOST: AEffectId = AEffectId::Custom(2);
pub(crate) const STASIS_WEB_PROBE: AEffectId = AEffectId::Custom(3);
pub(crate) const MISSILE_FLIGHT_TIME: AEffectId = AEffectId::Custom(4);
pub(crate) const STABILITY_GENERATOR_ELECTRIC: AEffectId = AEffectId::Custom(5);
pub(crate) const STABILITY_GENERATOR_PLASMA: AEffectId = AEffectId::Custom(6);
pub(crate) const STABILITY_GENERATOR_EXOTIC: AEffectId = AEffectId::Custom(7);
pub(crate) const STABILITY_GENERATOR_GAMMA: AEffectId = AEffectId::Custom(8);
