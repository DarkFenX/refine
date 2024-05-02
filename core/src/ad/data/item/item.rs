use crate::{
    ad::{AItemEffectData, AItemKind},
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel},
    util::{Named, StMap},
};

/// Represents an adapted item.
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
pub struct AItem {
    /// Item ID.
    pub id: EItemId,
    /// Item type.
    pub kind: Option<AItemKind>,
    /// Item group ID.
    pub grp_id: EItemGrpId,
    /// Item category ID.
    pub cat_id: EItemCatId,
    /// Attribute values of the item.
    pub attr_vals: StMap<EAttrId, AttrVal>,
    /// Refers effects of the item.
    pub effect_datas: StMap<EEffectId, AItemEffectData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<EEffectId>,
    /// Skill requirement map.
    pub srqs: StMap<EItemId, SkillLevel>,
}
impl AItem {
    /// Make a new adapted item type out of passed data.
    pub(crate) fn new(
        id: EItemId,
        kind: Option<AItemKind>,
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attr_vals: StMap<EAttrId, AttrVal>,
        effect_datas: StMap<EEffectId, AItemEffectData>,
        defeff_id: Option<EEffectId>,
        srqs: StMap<EItemId, SkillLevel>,
    ) -> Self {
        Self {
            id,
            kind,
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
