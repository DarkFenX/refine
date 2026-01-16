// TODO: after everything is implemented, remove this and see what needs to be cleaned up
#![allow(dead_code)]

use crate::{
    ad::{ACustomEffectId, AEffectId},
    ed::EEffectId,
};

impl AEffectId {
    pub(crate) const SHIELD_BOOSTING: Self = Self::from_eid(EEffectId::SHIELD_BOOSTING);
    pub(crate) const MISSILE_LAUNCHING: Self = Self::from_eid(EEffectId::MISSILE_LAUNCHING);
    pub(crate) const TGT_ATTACK: Self = Self::from_eid(EEffectId::TGT_ATTACK);
    pub(crate) const LO_POWER: Self = Self::from_eid(EEffectId::LO_POWER);
    pub(crate) const HI_POWER: Self = Self::from_eid(EEffectId::HI_POWER);
    pub(crate) const MED_POWER: Self = Self::from_eid(EEffectId::MED_POWER);
    pub(crate) const ONLINE: Self = Self::from_eid(EEffectId::ONLINE);
    pub(crate) const MINING: Self = Self::from_eid(EEffectId::MINING);
    pub(crate) const STRUCTURE_REPAIR: Self = Self::from_eid(EEffectId::STRUCTURE_REPAIR);
    pub(crate) const ARMOR_REPAIR: Self = Self::from_eid(EEffectId::ARMOR_REPAIR);
    pub(crate) const PROJECTILE_FIRED: Self = Self::from_eid(EEffectId::PROJECTILE_FIRED);
    pub(crate) const EMP_WAVE: Self = Self::from_eid(EEffectId::EMP_WAVE);
    pub(crate) const LAUNCHER_FITTED: Self = Self::from_eid(EEffectId::LAUNCHER_FITTED);
    pub(crate) const TURRET_FITTED: Self = Self::from_eid(EEffectId::TURRET_FITTED);
    pub(crate) const POWER_BOOSTER: Self = Self::from_eid(EEffectId::POWER_BOOSTER);
    pub(crate) const MINING_LASER: Self = Self::from_eid(EEffectId::MINING_LASER);
    pub(crate) const USE_MISSILES: Self = Self::from_eid(EEffectId::USE_MISSILES);
    pub(crate) const DEFENDER_MISSILE_LAUNCHING: Self = Self::from_eid(EEffectId::DEFENDER_MISSILE_LAUNCHING);
    pub(crate) const FOF_MISSILE_LAUNCHING: Self = Self::from_eid(EEffectId::FOF_MISSILE_LAUNCHING);
    pub(crate) const MISSILE_EM_DMG_BONUS: Self = Self::from_eid(EEffectId::MISSILE_EM_DMG_BONUS);
    pub(crate) const MISSILE_EXPL_DMG_BONUS: Self = Self::from_eid(EEffectId::MISSILE_EXPL_DMG_BONUS);
    pub(crate) const MISSILE_THERM_DMG_BONUS: Self = Self::from_eid(EEffectId::MISSILE_THERM_DMG_BONUS);
    pub(crate) const MISSILE_KIN_DMG_BONUS: Self = Self::from_eid(EEffectId::MISSILE_KIN_DMG_BONUS);
    pub(crate) const CLOAKING_TARGETING_DELAY_BONUS: Self = Self::from_eid(EEffectId::CLOAKING_TARGETING_DELAY_BONUS);
    pub(crate) const DRONE_DMG_BONUS: Self = Self::from_eid(EEffectId::DRONE_DMG_BONUS);
    pub(crate) const SELF_ROF: Self = Self::from_eid(EEffectId::SELF_ROF);
    pub(crate) const JUMP_PORTAL_GENERATION: Self = Self::from_eid(EEffectId::JUMP_PORTAL_GENERATION);
    pub(crate) const RIG_SLOT: Self = Self::from_eid(EEffectId::RIG_SLOT);
    pub(crate) const MINING_CLOUDS: Self = Self::from_eid(EEffectId::MINING_CLOUDS);
    pub(crate) const BOMB_LAUNCHING: Self = Self::from_eid(EEffectId::BOMB_LAUNCHING);
    pub(crate) const JUMP_PORTAL_GENERATION_BO: Self = Self::from_eid(EEffectId::JUMP_PORTAL_GENERATION_BO);
    pub(crate) const HARDPOINT_MODIFIER_EFFECT: Self = Self::from_eid(EEffectId::HARDPOINT_MODIFIER_EFFECT);
    pub(crate) const SLOT_MODIFIER: Self = Self::from_eid(EEffectId::SLOT_MODIFIER);
    pub(crate) const WARP_DISRUPT_SPHERE: Self = Self::from_eid(EEffectId::WARP_DISRUPT_SPHERE);
    pub(crate) const SUPER_WEAPON_AMARR: Self = Self::from_eid(EEffectId::SUPER_WEAPON_AMARR);
    pub(crate) const SUPER_WEAPON_CALDARI: Self = Self::from_eid(EEffectId::SUPER_WEAPON_CALDARI);
    pub(crate) const SUPER_WEAPON_GALLENTE: Self = Self::from_eid(EEffectId::SUPER_WEAPON_GALLENTE);
    pub(crate) const SUPER_WEAPON_MINMATAR: Self = Self::from_eid(EEffectId::SUPER_WEAPON_MINMATAR);
    pub(crate) const MICRO_JUMP_DRIVE: Self = Self::from_eid(EEffectId::MICRO_JUMP_DRIVE);
    pub(crate) const ADAPTIVE_ARMOR_HARDENER: Self = Self::from_eid(EEffectId::ADAPTIVE_ARMOR_HARDENER);
    pub(crate) const FUELED_SHIELD_BOOSTING: Self = Self::from_eid(EEffectId::FUELED_SHIELD_BOOSTING);
    pub(crate) const FUELED_ARMOR_REPAIR: Self = Self::from_eid(EEffectId::FUELED_ARMOR_REPAIR);
    pub(crate) const SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER: Self =
        Self::from_eid(EEffectId::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER);
    pub(crate) const SHIP_MOD_REMOTE_HULL_REPAIRER: Self = Self::from_eid(EEffectId::SHIP_MOD_REMOTE_HULL_REPAIRER);
    pub(crate) const SHIP_MOD_REMOTE_SHIELD_BOOSTER: Self = Self::from_eid(EEffectId::SHIP_MOD_REMOTE_SHIELD_BOOSTER);
    pub(crate) const ENERGY_NEUT_FALLOFF: Self = Self::from_eid(EEffectId::ENERGY_NEUT_FALLOFF);
    pub(crate) const SHIP_MOD_REMOTE_ARMOR_REPAIRER: Self = Self::from_eid(EEffectId::SHIP_MOD_REMOTE_ARMOR_REPAIRER);
    pub(crate) const ENERGY_NOSF_FALLOFF: Self = Self::from_eid(EEffectId::ENERGY_NOSF_FALLOFF);
    pub(crate) const DOOMSDAY_SLASH: Self = Self::from_eid(EEffectId::DOOMSDAY_SLASH);
    pub(crate) const MICRO_JUMP_PORTAL_DRIVE: Self = Self::from_eid(EEffectId::MICRO_JUMP_PORTAL_DRIVE);
    pub(crate) const STRUCT_ENERGY_NEUT_FALLOFF: Self = Self::from_eid(EEffectId::STRUCT_ENERGY_NEUT_FALLOFF);
    pub(crate) const STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC: Self =
        Self::from_eid(EEffectId::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC);
    pub(crate) const SERVICE_SLOT: Self = Self::from_eid(EEffectId::SERVICE_SLOT);
    pub(crate) const REMOTE_SENSOR_DAMP_FALLOFF: Self = Self::from_eid(EEffectId::REMOTE_SENSOR_DAMP_FALLOFF);
    pub(crate) const SHIP_MOD_GUIDANCE_DISRUPTOR: Self = Self::from_eid(EEffectId::SHIP_MOD_GUIDANCE_DISRUPTOR);
    pub(crate) const SHIP_MOD_TRACKING_DISRUPTOR: Self = Self::from_eid(EEffectId::SHIP_MOD_TRACKING_DISRUPTOR);
    pub(crate) const REMOTE_TARGET_PAINT_FALLOFF: Self = Self::from_eid(EEffectId::REMOTE_TARGET_PAINT_FALLOFF);
    pub(crate) const REMOTE_WEBIFIER_FALLOFF: Self = Self::from_eid(EEffectId::REMOTE_WEBIFIER_FALLOFF);
    pub(crate) const REMOTE_SENSOR_BOOST_FALLOFF: Self = Self::from_eid(EEffectId::REMOTE_SENSOR_BOOST_FALLOFF);
    pub(crate) const SHIP_MOD_REMOTE_TRACKING_COMPUTER: Self =
        Self::from_eid(EEffectId::SHIP_MOD_REMOTE_TRACKING_COMPUTER);
    pub(crate) const FTR_ABIL_MISSILES: Self = Self::from_eid(EEffectId::FTR_ABIL_MISSILES);
    pub(crate) const FTR_ABIL_NEUT: Self = Self::from_eid(EEffectId::FTR_ABIL_NEUT);
    pub(crate) const FTR_ABIL_WEB: Self = Self::from_eid(EEffectId::FTR_ABIL_WEB);
    pub(crate) const FTR_ABIL_POINT: Self = Self::from_eid(EEffectId::FTR_ABIL_POINT);
    pub(crate) const FTR_ABIL_ECM: Self = Self::from_eid(EEffectId::FTR_ABIL_ECM);
    pub(crate) const FTR_ABIL_EVASION: Self = Self::from_eid(EEffectId::FTR_ABIL_EVASION);
    pub(crate) const FTR_ABIL_AB: Self = Self::from_eid(EEffectId::FTR_ABIL_AB);
    pub(crate) const FTR_ABIL_MWD: Self = Self::from_eid(EEffectId::FTR_ABIL_MWD);
    pub(crate) const FTR_ABIL_MJD: Self = Self::from_eid(EEffectId::FTR_ABIL_MJD);
    pub(crate) const POINT_DEFENSE: Self = Self::from_eid(EEffectId::POINT_DEFENSE);
    pub(crate) const LIGHTNING_WEAPON: Self = Self::from_eid(EEffectId::LIGHTNING_WEAPON);
    pub(crate) const FTR_ABIL_TACKLE: Self = Self::from_eid(EEffectId::FTR_ABIL_TACKLE);
    pub(crate) const FTR_ABIL_ATK_MISSILE: Self = Self::from_eid(EEffectId::FTR_ABIL_ATK_MISSILE);
    pub(crate) const REMOTE_ECM_FALLOFF: Self = Self::from_eid(EEffectId::REMOTE_ECM_FALLOFF);
    pub(crate) const DOOMSDAY_BEAM_DOT: Self = Self::from_eid(EEffectId::DOOMSDAY_BEAM_DOT);
    pub(crate) const DOOMSDAY_CONE_DOT: Self = Self::from_eid(EEffectId::DOOMSDAY_CONE_DOT);
    pub(crate) const DOOMSDAY_HOG: Self = Self::from_eid(EEffectId::DOOMSDAY_HOG);
    pub(crate) const DOOMSDAY_AOE_WEB: Self = Self::from_eid(EEffectId::DOOMSDAY_AOE_WEB);
    pub(crate) const DOOMSDAY_AOE_NEUT: Self = Self::from_eid(EEffectId::DOOMSDAY_AOE_NEUT);
    pub(crate) const DOOMSDAY_AOE_PAINT: Self = Self::from_eid(EEffectId::DOOMSDAY_AOE_PAINT);
    pub(crate) const DOOMSDAY_AOE_TRACK: Self = Self::from_eid(EEffectId::DOOMSDAY_AOE_TRACK);
    pub(crate) const DOOMSDAY_AOE_DAMP: Self = Self::from_eid(EEffectId::DOOMSDAY_AOE_DAMP);
    pub(crate) const EMERGENCY_HULL_ENERGIZER: Self = Self::from_eid(EEffectId::EMERGENCY_HULL_ENERGIZER);
    pub(crate) const FTR_ABIL_BOMB: Self = Self::from_eid(EEffectId::FTR_ABIL_BOMB);
    pub(crate) const DOOMSDAY_AOE_ECM: Self = Self::from_eid(EEffectId::DOOMSDAY_AOE_ECM);
    pub(crate) const FTR_ABIL_KAMIKAZE: Self = Self::from_eid(EEffectId::FTR_ABIL_KAMIKAZE);
    pub(crate) const SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER: Self =
        Self::from_eid(EEffectId::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER);
    pub(crate) const SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER: Self =
        Self::from_eid(EEffectId::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER);
    pub(crate) const STRUCT_MOD_EFFECT_STASIS_WEBIFIER: Self =
        Self::from_eid(EEffectId::STRUCT_MOD_EFFECT_STASIS_WEBIFIER);
    pub(crate) const STRUCT_MOD_EFFECT_TARGET_PAINTER: Self =
        Self::from_eid(EEffectId::STRUCT_MOD_EFFECT_TARGET_PAINTER);
    pub(crate) const STRUCT_MOD_EFFECT_REMOTE_SENSOR_DAMPENER: Self =
        Self::from_eid(EEffectId::STRUCT_MOD_EFFECT_REMOTE_SENSOR_DAMPENER);
    pub(crate) const STRUCT_MOD_EFFECT_ECM: Self = Self::from_eid(EEffectId::STRUCT_MOD_EFFECT_ECM);
    pub(crate) const STRUCT_MOD_EFFECT_WEAPON_DISRUPTION: Self =
        Self::from_eid(EEffectId::STRUCT_MOD_EFFECT_WEAPON_DISRUPTION);
    pub(crate) const NPC_ENTITY_REMOTE_ARMOR_REPAIRER: Self =
        Self::from_eid(EEffectId::NPC_ENTITY_REMOTE_ARMOR_REPAIRER);
    pub(crate) const NPC_ENTITY_REMOTE_SHIELD_BOOSTER: Self =
        Self::from_eid(EEffectId::NPC_ENTITY_REMOTE_SHIELD_BOOSTER);
    pub(crate) const NPC_ENTITY_REMOTE_HULL_REPAIRER: Self = Self::from_eid(EEffectId::NPC_ENTITY_REMOTE_HULL_REPAIRER);
    pub(crate) const REMOTE_TARGET_PAINT_ENTITY: Self = Self::from_eid(EEffectId::REMOTE_TARGET_PAINT_ENTITY);
    pub(crate) const REMOTE_SENSOR_DAMP_ENTITY: Self = Self::from_eid(EEffectId::REMOTE_SENSOR_DAMP_ENTITY);
    pub(crate) const NPC_ENTITY_WEAPON_DISRUPTOR: Self = Self::from_eid(EEffectId::NPC_ENTITY_WEAPON_DISRUPTOR);
    pub(crate) const REMOTE_WEBIFIER_ENTITY: Self = Self::from_eid(EEffectId::REMOTE_WEBIFIER_ENTITY);
    pub(crate) const ENTITY_ENERGY_NEUT_FALLOFF: Self = Self::from_eid(EEffectId::ENTITY_ENERGY_NEUT_FALLOFF);
    pub(crate) const ENTITY_ECM_FALLOFF: Self = Self::from_eid(EEffectId::ENTITY_ECM_FALLOFF);
    pub(crate) const ECM_BURST_JAMMER: Self = Self::from_eid(EEffectId::ECM_BURST_JAMMER);
    pub(crate) const MOD_BONUS_INDUSTRIAL_INVULNERABILITY: Self =
        Self::from_eid(EEffectId::MOD_BONUS_INDUSTRIAL_INVULNERABILITY);
    pub(crate) const MOD_BONUS_MICROWARPDRIVE: Self = Self::from_eid(EEffectId::MOD_BONUS_MICROWARPDRIVE);
    pub(crate) const MOD_BONUS_AFTERBURNER: Self = Self::from_eid(EEffectId::MOD_BONUS_AFTERBURNER);
    pub(crate) const MOD_BONUS_WARFARE_LINK_ARMOR: Self = Self::from_eid(EEffectId::MOD_BONUS_WARFARE_LINK_ARMOR);
    pub(crate) const MOD_BONUS_WARFARE_LINK_SHIELD: Self = Self::from_eid(EEffectId::MOD_BONUS_WARFARE_LINK_SHIELD);
    pub(crate) const MOD_BONUS_WARFARE_LINK_SKIRMISH: Self = Self::from_eid(EEffectId::MOD_BONUS_WARFARE_LINK_SKIRMISH);
    pub(crate) const MOD_BONUS_WARFARE_LINK_INFO: Self = Self::from_eid(EEffectId::MOD_BONUS_WARFARE_LINK_INFO);
    pub(crate) const MOD_BONUS_WARFARE_LINK_MINING: Self = Self::from_eid(EEffectId::MOD_BONUS_WARFARE_LINK_MINING);
    pub(crate) const MOD_TITAN_EFFECT_GENERATOR: Self = Self::from_eid(EEffectId::MOD_TITAN_EFFECT_GENERATOR);
    pub(crate) const SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT: Self =
        Self::from_eid(EEffectId::SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT);
    pub(crate) const SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT: Self =
        Self::from_eid(EEffectId::SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT);
    pub(crate) const TGT_DISINTEGRATOR_ATTACK: Self = Self::from_eid(EEffectId::TGT_DISINTEGRATOR_ATTACK);
    pub(crate) const AOE_BEACON_BIOLUMINESCENCE_CLOUD: Self =
        Self::from_eid(EEffectId::AOE_BEACON_BIOLUMINESCENCE_CLOUD);
    pub(crate) const AOE_BEACON_CAUSTIC_CLOUD: Self = Self::from_eid(EEffectId::AOE_BEACON_CAUSTIC_CLOUD);
    pub(crate) const AOE_BEACON_PULSE_01: Self = Self::from_eid(EEffectId::AOE_BEACON_PULSE_01);
    pub(crate) const AOE_BEACON_FILAMENT_CLOUD: Self = Self::from_eid(EEffectId::AOE_BEACON_FILAMENT_CLOUD);
    pub(crate) const WEATHER_CAUSTIC_TOXIN: Self = Self::from_eid(EEffectId::WEATHER_CAUSTIC_TOXIN);
    pub(crate) const WEATHER_DARKNESS: Self = Self::from_eid(EEffectId::WEATHER_DARKNESS);
    pub(crate) const WEATHER_ELECTRIC_STORM: Self = Self::from_eid(EEffectId::WEATHER_ELECTRIC_STORM);
    pub(crate) const WEATHER_INFERNAL: Self = Self::from_eid(EEffectId::WEATHER_INFERNAL);
    pub(crate) const WEATHER_XENON_GAS: Self = Self::from_eid(EEffectId::WEATHER_XENON_GAS);
    pub(crate) const SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER: Self =
        Self::from_eid(EEffectId::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER);
    pub(crate) const CHAIN_LIGHTNING: Self = Self::from_eid(EEffectId::CHAIN_LIGHTNING);
    pub(crate) const DEBUFF_LANCE: Self = Self::from_eid(EEffectId::DEBUFF_LANCE);
    pub(crate) const MICRO_JUMP_PORTAL_DRIVE_CAPITAL: Self = Self::from_eid(EEffectId::MICRO_JUMP_PORTAL_DRIVE_CAPITAL);
    pub(crate) const DOT_MISSILE_LAUNCHING: Self = Self::from_eid(EEffectId::DOT_MISSILE_LAUNCHING);
    // Library-specific effects
    pub(crate) const CHAR_MISSILE_DMG: Self = Self::Custom(ACustomEffectId::from_i32(1));
    pub(crate) const AAR_PASTE_BOOST: Self = Self::Custom(ACustomEffectId::from_i32(2));
    pub(crate) const MISSILE_FLIGHT_TIME: Self = Self::Custom(ACustomEffectId::from_i32(3));
    pub(crate) const WARP_DISRUPT_PROBE: Self = Self::Custom(ACustomEffectId::from_i32(4));
    pub(crate) const STASIS_WEB_PROBE: Self = Self::Custom(ACustomEffectId::from_i32(5));
    pub(crate) const WDFG_SCRIPT_DEBUBBLE: Self = Self::Custom(ACustomEffectId::from_i32(6));
    pub(crate) const STABILITY_GENERATOR_ELECTRIC: Self = Self::Custom(ACustomEffectId::from_i32(7));
    pub(crate) const STABILITY_GENERATOR_PLASMA: Self = Self::Custom(ACustomEffectId::from_i32(8));
    pub(crate) const STABILITY_GENERATOR_EXOTIC: Self = Self::Custom(ACustomEffectId::from_i32(9));
    pub(crate) const STABILITY_GENERATOR_GAMMA: Self = Self::Custom(ACustomEffectId::from_i32(10));
}
