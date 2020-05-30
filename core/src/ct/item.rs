use std::collections::HashMap;

use crate::{
    consts::ItemType,
    defines::{ReeFloat, ReeInt},
};

/// Represents an item.
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
#[derive(Debug)]
pub struct Item {
    /// Item ID.
    pub id: ReeInt,
    /// Item type.
    pub itype: ItemType,
    /// Item group ID.
    pub grp_id: ReeInt,
    /// Item category ID.
    pub cat_id: ReeInt,
    /// Attribute values of the item.
    pub attr_vals: HashMap<ReeInt, ReeFloat>,
    /// Refers effects of the item.
    pub effect_ids: Vec<ReeInt>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<ReeInt>,
    /// Fighter ability properties specific to the item.
    pub abil_data: HashMap<ReeInt, FighterAbilData>,
    /// Skill requirement map.
    pub srqs: HashMap<ReeInt, ReeInt>,
}
impl Item {
    /// Make a new item out of passed data.
    pub fn new(
        id: ReeInt,
        itype: ItemType,
        grp_id: ReeInt,
        cat_id: ReeInt,
        attr_vals: HashMap<ReeInt, ReeFloat>,
        effect_ids: Vec<ReeInt>,
        defeff_id: Option<ReeInt>,
        abil_data: HashMap<ReeInt, FighterAbilData>,
        srqs: HashMap<ReeInt, ReeInt>,
    ) -> Item {
        Item {
            id,
            itype,
            grp_id,
            cat_id,
            attr_vals,
            effect_ids,
            defeff_id,
            abil_data,
            srqs,
        }
    }
}

/// Stores item-specific fighter ability data.
#[derive(Debug)]
pub struct FighterAbilData {
    /// Defines cooldown of the ability in seconds.
    pub cd: Option<ReeFloat>,
    /// Defines how many times the ability can be used before the fighter has to rearm.
    pub charges: Option<ReeInt>,
    /// Defines how long each charge of the ability takes to rearm, in seconds.
    pub rearm_time: Option<ReeFloat>,
}
impl FighterAbilData {
    /// Make a new per-item fighter ability data container out of passed data.
    pub fn new(cd: Option<ReeFloat>, charges: Option<ReeInt>, rearm_time: Option<ReeFloat>) -> FighterAbilData {
        FighterAbilData {
            cd,
            charges,
            rearm_time,
        }
    }
}
