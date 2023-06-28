#![allow(dead_code)]

use crate::defs::{AbilId, EffectId};

pub(crate) mod itemgrps {
    use crate::defs::ItemGrpId;

    pub(crate) const CHARACTER: ItemGrpId = 1;
    pub(crate) const EFFECT_BEACON: ItemGrpId = 920;
    pub(crate) const MUTAPLASMID: ItemGrpId = 1964;
    pub(crate) const SHIP_MOD: ItemGrpId = 1306;
}

pub(crate) mod itemcats {
    use crate::defs::ItemCatId;

    pub(crate) const CHARGE: ItemCatId = 8;
    pub(crate) const DRONE: ItemCatId = 18;
    pub(crate) const FIGHTER: ItemCatId = 87;
    pub(crate) const IMPLANT: ItemCatId = 20;
    pub(crate) const MODULE: ItemCatId = 7;
    pub(crate) const SHIP: ItemCatId = 6;
    pub(crate) const SKILL: ItemCatId = 16;
    pub(crate) const SUBSYSTEM: ItemCatId = 32;
}

pub(crate) mod attrs {
    use crate::defs::AttrId;

    // Fitting resource-related
    pub(crate) const CPU: AttrId = 50;
    pub(crate) const CPU_OUTPUT: AttrId = 48;
    pub(crate) const POWER: AttrId = 30;
    pub(crate) const POWER_OUTPUT: AttrId = 11;
    // Slot-related
    pub(crate) const BOOSTERNESS: AttrId = 1087;
    pub(crate) const IMPLANTNESS: AttrId = 331;
    pub(crate) const SUBSYSTEM_SLOT: AttrId = 1366;
    // Fighter-related
    pub(crate) const FTR_SQ_IS_HEAVY: AttrId = 2214;
    pub(crate) const FTR_SQ_IS_LIGHT: AttrId = 2212;
    pub(crate) const FTR_SQ_IS_SUPPORT: AttrId = 2213;
    // Buff-related
    pub(crate) const WARFARE_BUFF1_ID: AttrId = 2468;
    pub(crate) const WARFARE_BUFF2_ID: AttrId = 2470;
    pub(crate) const WARFARE_BUFF3_ID: AttrId = 2472;
    pub(crate) const WARFARE_BUFF4_ID: AttrId = 2536;

    pub(crate) const BUFF_ID_ATTRS: [AttrId; 4] =
        [WARFARE_BUFF1_ID, WARFARE_BUFF2_ID, WARFARE_BUFF3_ID, WARFARE_BUFF4_ID];
}

pub(crate) mod effects {
    use crate::defs::EffectId;

    pub(crate) const HI_POWER: EffectId = 12;
    pub(crate) const LO_POWER: EffectId = 11;
    pub(crate) const MED_POWER: EffectId = 13;
    pub(crate) const RIG_SLOT: EffectId = 2663;
    pub(crate) const SUBSYSTEM: EffectId = 3772;
    // Fighter-related
    pub(crate) const FTR_ABIL_MISSILES: EffectId = 6431;
    pub(crate) const FTR_ABIL_NEUT: EffectId = 6434;
    pub(crate) const FTR_ABIL_WEB: EffectId = 6435;
    pub(crate) const FTR_ABIL_POINT: EffectId = 6436;
    pub(crate) const FTR_ABIL_ECM: EffectId = 6437;
    pub(crate) const FTR_ABIL_EVASION: EffectId = 6439;
    pub(crate) const FTR_ABIL_AB: EffectId = 6440;
    pub(crate) const FTR_ABIL_MWD: EffectId = 6441;
    pub(crate) const FTR_ABIL_MJD: EffectId = 6442;
    pub(crate) const FTR_ABIL_TACKLE: EffectId = 6464;
    pub(crate) const FTR_ABIL_ATK_MISSILE: EffectId = 6465;
    pub(crate) const FTR_ABIL_BOMB: EffectId = 6485;
    pub(crate) const FTR_ABIL_KAMIKAZE: EffectId = 6554;
}

pub(crate) mod effcats {
    use crate::defs::EffectCatId;

    pub(crate) const PASSIVE: EffectCatId = 0;
    pub(crate) const ACTIVE: EffectCatId = 1;
    pub(crate) const TARGET: EffectCatId = 2;
    pub(crate) const AREA: EffectCatId = 3;
    pub(crate) const ONLINE: EffectCatId = 4;
    pub(crate) const OVERLOAD: EffectCatId = 5;
    pub(crate) const DUNGEON: EffectCatId = 6;
    pub(crate) const SYSTEM: EffectCatId = 7;
}

pub(crate) mod units {
    use crate::defs::AttrUnitId;

    pub(crate) const GROUP_ID: AttrUnitId = 115;
    pub(crate) const ITEM_ID: AttrUnitId = 116;
    pub(crate) const ATTR_ID: AttrUnitId = 119;
}

pub(crate) mod abils {
    use crate::defs::AbilId;

    pub(crate) const WEB: AbilId = 2;
    pub(crate) const MWD: AbilId = 4;
    pub(crate) const MJD: AbilId = 5;
    pub(crate) const BOMB: AbilId = 7;
    pub(crate) const AB: AbilId = 9;
    pub(crate) const POINT: AbilId = 10;
    pub(crate) const NEUT: AbilId = 11;
    pub(crate) const ECM: AbilId = 12;
    pub(crate) const EVASION: AbilId = 13;
    pub(crate) const TACKLE: AbilId = 16;
    pub(crate) const TORP_EM: AbilId = 18;
    pub(crate) const TORP_THERM: AbilId = 19;
    pub(crate) const TORP_KIN: AbilId = 20;
    pub(crate) const TORP_EXP: AbilId = 21;
    pub(crate) const ATK_PULSE: AbilId = 22;
    pub(crate) const ATK_BEAM: AbilId = 23;
    pub(crate) const ATK_BLASTER_THERM: AbilId = 24;
    pub(crate) const ATK_RAIL_THERM: AbilId = 25;
    pub(crate) const ATK_AUTOCANNON: AbilId = 26;
    pub(crate) const ATK_ARTY: AbilId = 27;
    pub(crate) const UMISSILE_EM: AbilId = 29;
    pub(crate) const UMISSILE_THERM: AbilId = 30;
    pub(crate) const UMISSILE_KIN: AbilId = 31;
    pub(crate) const UMISSILE_EXP: AbilId = 32;
    pub(crate) const ROCKET_EM: AbilId = 33;
    pub(crate) const ROCKET_THERM: AbilId = 34;
    pub(crate) const ROCKET_KIN: AbilId = 35;
    pub(crate) const ROCKET_EXP: AbilId = 36;
    pub(crate) const KAMIKAZE: AbilId = 38;
    pub(crate) const ATK_BLASTER_KIN: AbilId = 44;
    pub(crate) const ATK_RAIL_KIN: AbilId = 45;
}

pub(crate) fn get_abil_effect(abil_id: AbilId) -> Option<EffectId> {
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
