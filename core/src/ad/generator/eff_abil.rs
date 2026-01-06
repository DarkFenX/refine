use crate::ed::{EAbilId, EEffectId};

pub(in crate::ad::generator) fn get_abil_effect(abil_id: EAbilId) -> Option<EEffectId> {
    match abil_id {
        EAbilId::ATK_PULSE
        | EAbilId::ATK_BEAM
        | EAbilId::ATK_BLASTER_THERM
        | EAbilId::ATK_BLASTER_KIN
        | EAbilId::ATK_RAIL_THERM
        | EAbilId::ATK_RAIL_KIN
        | EAbilId::ATK_AUTOCANNON
        | EAbilId::ATK_ARTY => Some(EEffectId::FTR_ABIL_ATK_MISSILE),
        EAbilId::UMISSILE_EM
        | EAbilId::UMISSILE_THERM
        | EAbilId::UMISSILE_KIN
        | EAbilId::UMISSILE_EXP
        | EAbilId::ROCKET_EM
        | EAbilId::ROCKET_THERM
        | EAbilId::ROCKET_KIN
        | EAbilId::ROCKET_EXP
        | EAbilId::TORP_EM
        | EAbilId::TORP_THERM
        | EAbilId::TORP_KIN
        | EAbilId::TORP_EXP => Some(EEffectId::FTR_ABIL_MISSILES),
        EAbilId::BOMB => Some(EEffectId::FTR_ABIL_BOMB),
        EAbilId::KAMIKAZE => Some(EEffectId::FTR_ABIL_KAMIKAZE),
        EAbilId::POINT => Some(EEffectId::FTR_ABIL_POINT),
        EAbilId::WEB => Some(EEffectId::FTR_ABIL_WEB),
        EAbilId::TACKLE => Some(EEffectId::FTR_ABIL_TACKLE),
        EAbilId::NEUT => Some(EEffectId::FTR_ABIL_NEUT),
        EAbilId::ECM => Some(EEffectId::FTR_ABIL_ECM),
        EAbilId::MWD => Some(EEffectId::FTR_ABIL_MWD),
        EAbilId::AB => Some(EEffectId::FTR_ABIL_AB),
        EAbilId::MJD => Some(EEffectId::FTR_ABIL_MJD),
        EAbilId::EVASION => Some(EEffectId::FTR_ABIL_EVASION),
        _ => None,
    }
}
