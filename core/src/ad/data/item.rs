use std::collections::HashMap;

use crate::{
    consts::ItemType,
    defs::{Amount, AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel},
    util::Named,
};

/// Represents an adapted item type.
///
/// An item type carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
#[derive(Debug)]
pub struct AItem {
    /// Item ID.
    pub id: EItemId,
    /// Item type.
    pub itype: Option<ItemType>,
    /// Item group ID.
    pub grp_id: EItemGrpId,
    /// Item category ID.
    pub cat_id: EItemCatId,
    /// Attribute values of the item.
    pub attr_vals: HashMap<EAttrId, AttrVal>,
    /// Refers effects of the item.
    pub effect_datas: HashMap<EEffectId, AItemEffData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<EEffectId>,
    /// Skill requirement map.
    pub srqs: HashMap<EItemId, SkillLevel>,
}
impl AItem {
    /// Make a new adapted item type out of passed data.
    pub(crate) fn new(
        id: EItemId,
        itype: Option<ItemType>,
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attr_vals: HashMap<EAttrId, AttrVal>,
        effect_datas: HashMap<EEffectId, AItemEffData>,
        defeff_id: Option<EEffectId>,
        srqs: HashMap<EItemId, SkillLevel>,
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
