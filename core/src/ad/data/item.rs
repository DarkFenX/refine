use std::collections::HashMap;

use crate::{
    consts::ItemType,
    defs::{Amount, AttrId, AttrVal, EffectId, ItemCatId, ItemGrpId, ItemId, SkillLevel},
    util::Named,
};

/// Represents an adapted item type.
///
/// An item type carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
#[derive(Debug)]
pub struct AItem {
    /// Item ID.
    pub id: ItemId,
    /// Item type.
    pub itype: Option<ItemType>,
    /// Item group ID.
    pub grp_id: ItemGrpId,
    /// Item category ID.
    pub cat_id: ItemCatId,
    /// Attribute values of the item.
    pub attr_vals: HashMap<AttrId, AttrVal>,
    /// Refers effects of the item.
    pub effect_datas: HashMap<EffectId, AItemEffData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<EffectId>,
    /// Skill requirement map.
    pub srqs: HashMap<ItemId, SkillLevel>,
}
impl AItem {
    /// Make a new adapted item type out of passed data.
    pub(crate) fn new(
        id: ItemId,
        itype: Option<ItemType>,
        grp_id: ItemGrpId,
        cat_id: ItemCatId,
        attr_vals: HashMap<AttrId, AttrVal>,
        effect_datas: HashMap<EffectId, AItemEffData>,
        defeff_id: Option<EffectId>,
        srqs: HashMap<ItemId, SkillLevel>,
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
        "AItem"
    }
}

/// Stores item-specific effect data.
#[derive(Debug)]
pub struct AItemEffData {
    /// Defines cooldown of the effect in seconds.
    pub cd: Option<AttrVal>,
    /// Defines how many times the effect can be used before its parent item has to reload.
    pub charge_amount: Option<Amount>,
    /// Defines how much time each charge of the effect takes to reload, in seconds.
    pub charge_reload_time: Option<AttrVal>,
}
impl AItemEffData {
    /// Make a new per-item effect data container out of passed data.
    pub(crate) fn new(cd: Option<AttrVal>, charge_amount: Option<Amount>, charge_reload_time: Option<AttrVal>) -> Self {
        Self {
            cd,
            charge_amount,
            charge_reload_time,
        }
    }
}
impl Named for AItemEffData {
    fn get_name() -> &'static str {
        "AItemEffData"
    }
}
