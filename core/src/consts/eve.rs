#![allow(dead_code)]

use crate::defs::{EAbilId, EEffectId};

pub(crate) mod itemgrps {
    use crate::defs::EItemGrpId;

    pub(crate) const CHARACTER: EItemGrpId = 1;
    pub(crate) const EFFECT_BEACON: EItemGrpId = 920;
    pub(crate) const MUTAPLASMID: EItemGrpId = 1964;
    pub(crate) const SHIP_MOD: EItemGrpId = 1306;
}

pub(crate) mod itemcats {
    use crate::defs::EItemCatId;

    pub(crate) const CHARGE: EItemCatId = 8;
    pub(crate) const DRONE: EItemCatId = 18;
    pub(crate) const FIGHTER: EItemCatId = 87;
    pub(crate) const IMPLANT: EItemCatId = 20;
    pub(crate) const MODULE: EItemCatId = 7;
    pub(crate) const SHIP: EItemCatId = 6;
    pub(crate) const SKILL: EItemCatId = 16;
    pub(crate) const SUBSYSTEM: EItemCatId = 32;
}

pub(crate) mod attrs {
    use crate::defs::EAttrId;

    pub(crate) const SKILL_LEVEL: EAttrId = 280;
    // Fitting resource-related
    pub(crate) const CPU: EAttrId = 50;
    pub(crate) const CPU_OUTPUT: EAttrId = 48;
    pub(crate) const POWER: EAttrId = 30;
    pub(crate) const POWER_OUTPUT: EAttrId = 11;
    // Slot-related
    pub(crate) const BOOSTERNESS: EAttrId = 1087;
    pub(crate) const IMPLANTNESS: EAttrId = 331;
    pub(crate) const SUBSYSTEM_SLOT: EAttrId = 1366;
    // Fighter-related
    pub(crate) const FTR_SQ_IS_HEAVY: EAttrId = 2214;
    pub(crate) const FTR_SQ_IS_LIGHT: EAttrId = 2212;
    pub(crate) const FTR_SQ_IS_SUPPORT: EAttrId = 2213;
    // Buff-related
    pub(crate) const WARFARE_BUFF1_ID: EAttrId = 2468;
    pub(crate) const WARFARE_BUFF2_ID: EAttrId = 2470;
    pub(crate) const WARFARE_BUFF3_ID: EAttrId = 2472;
    pub(crate) const WARFARE_BUFF4_ID: EAttrId = 2536;

    pub(crate) const BUFF_ID_ATTRS: [EAttrId; 4] =
        [WARFARE_BUFF1_ID, WARFARE_BUFF2_ID, WARFARE_BUFF3_ID, WARFARE_BUFF4_ID];
}

pub(crate) mod effects {
    use crate::defs::EEffectId;

    pub(crate) const ONLINE: EEffectId = 16;
    // Slot-related
    pub(crate) const HI_POWER: EEffectId = 12;
    pub(crate) const LO_POWER: EEffectId = 11;
    pub(crate) const MED_POWER: EEffectId = 13;
    pub(crate) const RIG_SLOT: EEffectId = 2663;
    pub(crate) const SUBSYSTEM: EEffectId = 3772;
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
}

pub(crate) mod effcats {
    use crate::defs::EEffectCatId;

    pub(crate) const PASSIVE: EEffectCatId = 0;
    pub(crate) const ACTIVE: EEffectCatId = 1;
    pub(crate) const TARGET: EEffectCatId = 2;
    pub(crate) const AREA: EEffectCatId = 3;
    pub(crate) const ONLINE: EEffectCatId = 4;
    pub(crate) const OVERLOAD: EEffectCatId = 5;
    pub(crate) const DUNGEON: EEffectCatId = 6;
    pub(crate) const SYSTEM: EEffectCatId = 7;
}

pub(crate) mod units {
    use crate::defs::EAttrUnitId;

    pub(crate) const GROUP_ID: EAttrUnitId = 115;
    pub(crate) const ITEM_ID: EAttrUnitId = 116;
    pub(crate) const ATTR_ID: EAttrUnitId = 119;
}

pub(crate) mod abils {
    use crate::defs::EAbilId;

    pub(crate) const WEB: EAbilId = 2;
    pub(crate) const MWD: EAbilId = 4;
    pub(crate) const MJD: EAbilId = 5;
    pub(crate) const BOMB: EAbilId = 7;
    pub(crate) const AB: EAbilId = 9;
    pub(crate) const POINT: EAbilId = 10;
    pub(crate) const NEUT: EAbilId = 11;
    pub(crate) const ECM: EAbilId = 12;
    pub(crate) const EVASION: EAbilId = 13;
    pub(crate) const TACKLE: EAbilId = 16;
    pub(crate) const TORP_EM: EAbilId = 18;
    pub(crate) const TORP_THERM: EAbilId = 19;
    pub(crate) const TORP_KIN: EAbilId = 20;
    pub(crate) const TORP_EXP: EAbilId = 21;
    pub(crate) const ATK_PULSE: EAbilId = 22;
    pub(crate) const ATK_BEAM: EAbilId = 23;
    pub(crate) const ATK_BLASTER_THERM: EAbilId = 24;
    pub(crate) const ATK_RAIL_THERM: EAbilId = 25;
    pub(crate) const ATK_AUTOCANNON: EAbilId = 26;
    pub(crate) const ATK_ARTY: EAbilId = 27;
    pub(crate) const UMISSILE_EM: EAbilId = 29;
    pub(crate) const UMISSILE_THERM: EAbilId = 30;
    pub(crate) const UMISSILE_KIN: EAbilId = 31;
    pub(crate) const UMISSILE_EXP: EAbilId = 32;
    pub(crate) const ROCKET_EM: EAbilId = 33;
    pub(crate) const ROCKET_THERM: EAbilId = 34;
    pub(crate) const ROCKET_KIN: EAbilId = 35;
    pub(crate) const ROCKET_EXP: EAbilId = 36;
    pub(crate) const KAMIKAZE: EAbilId = 38;
    pub(crate) const ATK_BLASTER_KIN: EAbilId = 44;
    pub(crate) const ATK_RAIL_KIN: EAbilId = 45;
}

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
