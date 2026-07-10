use crate::ed::{EAbilId, EEffectId};

pub(in crate::ad::generator) fn get_abil_effect(abil_id: EAbilId) -> Option<EEffectId> {
    match abil_id {
        EAbilId::PULSE_CANNON
        | EAbilId::BEAM_CANNON
        | EAbilId::BLASTER_CANNON_THERM
        | EAbilId::BLASTER_CANNON_KIN
        | EAbilId::RAILGUN_THERM
        | EAbilId::RAILGUN_KIN
        | EAbilId::AUTOCANNON
        | EAbilId::ARTILLERY => Some(EEffectId::FTR_ABIL_ATTACK_M),
        EAbilId::UMISSILE_SWARM_EM
        | EAbilId::UMISSILE_SWARM_THERM
        | EAbilId::UMISSILE_SWARM_KIN
        | EAbilId::UMISSILE_SWARM_EXP
        | EAbilId::HEAVY_ROCKET_SALVO_EM
        | EAbilId::HEAVY_ROCKET_SALVO_THERM
        | EAbilId::HEAVY_ROCKET_SALVO_KIN
        | EAbilId::HEAVY_ROCKET_SALVO_EXP
        | EAbilId::TORPEDO_SALVO_EM
        | EAbilId::TORPEDO_SALVO_THERM
        | EAbilId::TORPEDO_SALVO_KIN
        | EAbilId::TORPEDO_SALVO_EXP => Some(EEffectId::FTR_ABIL_MISSILES),
        EAbilId::LAUNCH_BOMB => Some(EEffectId::FTR_ABIL_LAUNCH_BOMB),
        EAbilId::TRUE_SACRIFICE => Some(EEffectId::FTR_ABIL_KAMIKAZE),
        EAbilId::WARP_DISRUPT => Some(EEffectId::FTR_ABIL_WARP_DISRUPT),
        EAbilId::STASIS_WEB => Some(EEffectId::FTR_ABIL_STASIS_WEB),
        EAbilId::TACKLE => Some(EEffectId::FTR_ABIL_TACKLE),
        EAbilId::ENERGY_NEUT => Some(EEffectId::FTR_ABIL_ENERGY_NEUT),
        EAbilId::ECM => Some(EEffectId::FTR_ABIL_ECM),
        EAbilId::MICRO_WARP_DRIVE => Some(EEffectId::FTR_ABIL_MICRO_WARP_DRIVE),
        EAbilId::AFTERBURNER => Some(EEffectId::FTR_ABIL_AFTERBURNER),
        EAbilId::MICRO_JUMP_DRIVE => Some(EEffectId::FTR_ABIL_MICRO_JUMP_DRIVE),
        EAbilId::EVASIVE_MANEUVERS => Some(EEffectId::FTR_ABIL_EVASIVE_MANEUVERS),
        _ => None,
    }
}
