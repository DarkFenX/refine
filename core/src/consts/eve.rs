#![allow(dead_code)]

use crate::defines::ReeInt;

pub(crate) mod itemgrps {
    use crate::defines::ReeInt;

    pub(crate) const CHARACTER: ReeInt = 1;
    pub(crate) const EFFECT_BEACON: ReeInt = 920;
}

pub(crate) mod itemcats {
    use crate::defines::ReeInt;

    pub(crate) const CHARGE: ReeInt = 8;
    pub(crate) const DRONE: ReeInt = 18;
    pub(crate) const FIGHTER: ReeInt = 87;
    pub(crate) const IMPLANT: ReeInt = 20;
    pub(crate) const MODULE: ReeInt = 7;
    pub(crate) const SHIP: ReeInt = 6;
    pub(crate) const SKILL: ReeInt = 16;
    pub(crate) const SUBSYSTEM: ReeInt = 32;
}

pub(crate) mod attrs {
    use crate::defines::ReeInt;

    pub(crate) const WARFARE_BUFF1_ID: ReeInt = 2468;
    pub(crate) const WARFARE_BUFF2_ID: ReeInt = 2470;
    pub(crate) const WARFARE_BUFF3_ID: ReeInt = 2472;
    pub(crate) const WARFARE_BUFF4_ID: ReeInt = 2536;

    pub(crate) const BUFF_ID_ATTRS: [ReeInt; 4] =
        [WARFARE_BUFF1_ID, WARFARE_BUFF2_ID, WARFARE_BUFF3_ID, WARFARE_BUFF4_ID];
}

pub(crate) mod effects {
    use crate::defines::ReeInt;

    // Fighter-related
    pub(crate) const FTR_ABIL_MISSILES: ReeInt = 6431;
    pub(crate) const FTR_ABIL_NEUT: ReeInt = 6434;
    pub(crate) const FTR_ABIL_WEB: ReeInt = 6435;
    pub(crate) const FTR_ABIL_POINT: ReeInt = 6436;
    pub(crate) const FTR_ABIL_ECM: ReeInt = 6437;
    pub(crate) const FTR_ABIL_EVASION: ReeInt = 6439;
    pub(crate) const FTR_ABIL_AB: ReeInt = 6440;
    pub(crate) const FTR_ABIL_MWD: ReeInt = 6441;
    pub(crate) const FTR_ABIL_MJD: ReeInt = 6442;
    pub(crate) const FTR_ABIL_TACKLE: ReeInt = 6464;
    pub(crate) const FTR_ABIL_ATK_MISSILE: ReeInt = 6465;
    pub(crate) const FTR_ABIL_BOMB: ReeInt = 6485;
    pub(crate) const FTR_ABIL_KAMIKAZE: ReeInt = 6554;
}

pub(crate) mod units {
    use crate::defines::ReeInt;

    pub(crate) const GROUP_ID: ReeInt = 115;
    pub(crate) const ITEM_ID: ReeInt = 116;
    pub(crate) const ATTR_ID: ReeInt = 119;
}

pub(crate) mod abils {
    use crate::defines::ReeInt;

    pub(crate) const WEB: ReeInt = 2;
    pub(crate) const MWD: ReeInt = 4;
    pub(crate) const MJD: ReeInt = 5;
    pub(crate) const BOMB: ReeInt = 7;
    pub(crate) const AB: ReeInt = 9;
    pub(crate) const POINT: ReeInt = 10;
    pub(crate) const NEUT: ReeInt = 11;
    pub(crate) const ECM: ReeInt = 12;
    pub(crate) const EVASION: ReeInt = 13;
    pub(crate) const TACKLE: ReeInt = 16;
    pub(crate) const TORP_EM: ReeInt = 18;
    pub(crate) const TORP_THERM: ReeInt = 19;
    pub(crate) const TORP_KIN: ReeInt = 20;
    pub(crate) const TORP_EXP: ReeInt = 21;
    pub(crate) const ATK_PULSE: ReeInt = 22;
    pub(crate) const ATK_BEAM: ReeInt = 23;
    pub(crate) const ATK_BLASTER_THERM: ReeInt = 24;
    pub(crate) const ATK_RAIL_THERM: ReeInt = 25;
    pub(crate) const ATK_AUTOCANNON: ReeInt = 26;
    pub(crate) const ATK_ARTY: ReeInt = 27;
    pub(crate) const UMISSILE_EM: ReeInt = 29;
    pub(crate) const UMISSILE_THERM: ReeInt = 30;
    pub(crate) const UMISSILE_KIN: ReeInt = 31;
    pub(crate) const UMISSILE_EXP: ReeInt = 32;
    pub(crate) const ROCKET_EM: ReeInt = 33;
    pub(crate) const ROCKET_THERM: ReeInt = 34;
    pub(crate) const ROCKET_KIN: ReeInt = 35;
    pub(crate) const ROCKET_EXP: ReeInt = 36;
    pub(crate) const KAMIKAZE: ReeInt = 38;
    pub(crate) const ATK_BLASTER_KIN: ReeInt = 44;
    pub(crate) const ATK_RAIL_KIN: ReeInt = 45;
}

pub(crate) fn get_abil_effect(abil_id: ReeInt) -> Option<ReeInt> {
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
