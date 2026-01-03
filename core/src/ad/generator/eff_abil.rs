use crate::{
    ec,
    ed::{EAbilId, EEffectId},
};

pub(in crate::ad::generator) fn get_abil_effect(abil_id: EAbilId) -> Option<EEffectId> {
    match abil_id {
        ec::abils::ATK_PULSE
        | ec::abils::ATK_BEAM
        | ec::abils::ATK_BLASTER_THERM
        | ec::abils::ATK_BLASTER_KIN
        | ec::abils::ATK_RAIL_THERM
        | ec::abils::ATK_RAIL_KIN
        | ec::abils::ATK_AUTOCANNON
        | ec::abils::ATK_ARTY => Some(ec::effects::FTR_ABIL_ATK_MISSILE),
        ec::abils::UMISSILE_EM
        | ec::abils::UMISSILE_THERM
        | ec::abils::UMISSILE_KIN
        | ec::abils::UMISSILE_EXP
        | ec::abils::ROCKET_EM
        | ec::abils::ROCKET_THERM
        | ec::abils::ROCKET_KIN
        | ec::abils::ROCKET_EXP
        | ec::abils::TORP_EM
        | ec::abils::TORP_THERM
        | ec::abils::TORP_KIN
        | ec::abils::TORP_EXP => Some(ec::effects::FTR_ABIL_MISSILES),
        ec::abils::BOMB => Some(ec::effects::FTR_ABIL_BOMB),
        ec::abils::KAMIKAZE => Some(ec::effects::FTR_ABIL_KAMIKAZE),
        ec::abils::POINT => Some(ec::effects::FTR_ABIL_POINT),
        ec::abils::WEB => Some(ec::effects::FTR_ABIL_WEB),
        ec::abils::TACKLE => Some(ec::effects::FTR_ABIL_TACKLE),
        ec::abils::NEUT => Some(ec::effects::FTR_ABIL_NEUT),
        ec::abils::ECM => Some(ec::effects::FTR_ABIL_ECM),
        ec::abils::MWD => Some(ec::effects::FTR_ABIL_MWD),
        ec::abils::AB => Some(ec::effects::FTR_ABIL_AB),
        ec::abils::MJD => Some(ec::effects::FTR_ABIL_MJD),
        ec::abils::EVASION => Some(ec::effects::FTR_ABIL_EVASION),
        _ => None,
    }
}
