use crate::{
    defs::{AttrVal, EAbilId, EAttrId, EEffectId, OF},
    ec::{abils, attrs, effects},
};

pub(crate) const MAX_SUBCAP_MODULE_VOLUME: AttrVal = OF(3500.0);

pub(crate) const BUFF_STDATTR_IDS: [EAttrId; 4] = [
    attrs::WARFARE_BUFF1_ID,
    attrs::WARFARE_BUFF2_ID,
    attrs::WARFARE_BUFF3_ID,
    attrs::WARFARE_BUFF4_ID,
];
pub(crate) const BUFF_STDATTRS: [(EAttrId, EAttrId); 4] = [
    (attrs::WARFARE_BUFF1_ID, attrs::WARFARE_BUFF1_VAL),
    (attrs::WARFARE_BUFF2_ID, attrs::WARFARE_BUFF2_VAL),
    (attrs::WARFARE_BUFF3_ID, attrs::WARFARE_BUFF3_VAL),
    (attrs::WARFARE_BUFF4_ID, attrs::WARFARE_BUFF4_VAL),
];

pub(crate) fn get_abil_effect(abil_id: EAbilId) -> Option<EEffectId> {
    match abil_id {
        abils::ATK_PULSE
        | abils::ATK_BEAM
        | abils::ATK_BLASTER_THERM
        | abils::ATK_BLASTER_KIN
        | abils::ATK_RAIL_THERM
        | abils::ATK_RAIL_KIN
        | abils::ATK_AUTOCANNON
        | abils::ATK_ARTY => Some(effects::FTR_ABIL_ATK_MISSILE),
        abils::UMISSILE_EM
        | abils::UMISSILE_THERM
        | abils::UMISSILE_KIN
        | abils::UMISSILE_EXP
        | abils::ROCKET_EM
        | abils::ROCKET_THERM
        | abils::ROCKET_KIN
        | abils::ROCKET_EXP
        | abils::TORP_EM
        | abils::TORP_THERM
        | abils::TORP_KIN
        | abils::TORP_EXP => Some(effects::FTR_ABIL_MISSILES),
        abils::BOMB => Some(effects::FTR_ABIL_BOMB),
        abils::KAMIKAZE => Some(effects::FTR_ABIL_KAMIKAZE),
        abils::POINT => Some(effects::FTR_ABIL_POINT),
        abils::WEB => Some(effects::FTR_ABIL_WEB),
        abils::TACKLE => Some(effects::FTR_ABIL_TACKLE),
        abils::NEUT => Some(effects::FTR_ABIL_NEUT),
        abils::ECM => Some(effects::FTR_ABIL_ECM),
        abils::MWD => Some(effects::FTR_ABIL_MWD),
        abils::AB => Some(effects::FTR_ABIL_AB),
        abils::MJD => Some(effects::FTR_ABIL_MJD),
        abils::EVASION => Some(effects::FTR_ABIL_EVASION),
        _ => None,
    }
}
