use std::collections::HashMap;

use crate::{
    ad::{AItemEffData, AItemType},
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel},
    shr::FitType,
    util::Named,
};

/// Represents an adapted item.
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
pub struct AItem {
    /// Item ID.
    pub id: EItemId,
    /// Item type.
    pub itype: Option<AItemType>,
    /// Fit type for this item.
    pub ftype: Option<FitType>,
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
        itype: Option<AItemType>,
        ftype: Option<FitType>,
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
            ftype,
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
impl std::fmt::Display for AItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
