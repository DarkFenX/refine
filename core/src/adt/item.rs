use std::collections::HashMap;

use crate::{
    consts::ItemType,
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// Represents an item.
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
#[derive(Debug)]
pub struct AItem {
    /// Item ID.
    pub id: ReeInt,
    /// Item type.
    pub itype: Option<ItemType>,
    /// Item group ID.
    pub grp_id: ReeInt,
    /// Item category ID.
    pub cat_id: ReeInt,
    /// Attribute values of the item.
    pub attr_vals: HashMap<ReeInt, ReeFloat>,
    /// Refers effects of the item.
    pub effect_datas: HashMap<ReeInt, AItemEffData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<ReeInt>,
    /// Skill requirement map.
    pub srqs: HashMap<ReeInt, ReeInt>,
}
impl AItem {
    /// Make a new item out of passed data.
    pub(crate) fn new(
        id: ReeInt,
        itype: Option<ItemType>,
        grp_id: ReeInt,
        cat_id: ReeInt,
        attr_vals: HashMap<ReeInt, ReeFloat>,
        effect_datas: HashMap<ReeInt, AItemEffData>,
        defeff_id: Option<ReeInt>,
        srqs: HashMap<ReeInt, ReeInt>,
    ) -> Self {
        Self {
            id,
            itype,
            grp_id,
            cat_id,
            attr_vals,
            effect_datas,
            defeff_id,
            srqs,
        }
    }
}
impl Named for AItem {
    fn get_name() -> &'static str {
        "ct::Item"
    }
}

/// Stores item-specific effect data.
#[derive(Debug)]
pub struct AItemEffData {
    /// Defines cooldown of the effect in seconds.
    pub cd: Option<ReeFloat>,
    /// Defines how many times the effect can be used before its parent item has to reload.
    pub charge_amount: Option<ReeInt>,
    /// Defines how much time each charge of the effect takes to reload, in seconds.
    pub charge_reload_time: Option<ReeFloat>,
}
impl AItemEffData {
    /// Make a new per-item effect data container out of passed data.
    pub(crate) fn new(
        cd: Option<ReeFloat>,
        charge_amount: Option<ReeInt>,
        charge_reload_time: Option<ReeFloat>,
    ) -> Self {
        Self {
            cd,
            charge_amount,
            charge_reload_time,
        }
    }
}
impl Named for AItemEffData {
    fn get_name() -> &'static str {
        "ct::ItemEffData"
    }
}
