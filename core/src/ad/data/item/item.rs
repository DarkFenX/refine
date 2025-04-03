use crate::{
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemExtras, AItemGrpId, AItemId, ASkillLevel},
    util::{HMap, Named},
};

/// Represents an adapted item.
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
pub struct AItem {
    /// Item ID.
    pub id: AItemId,
    /// Item group ID.
    pub grp_id: AItemGrpId,
    /// Item category ID.
    pub cat_id: AItemCatId,
    /// Attribute values of the item.
    pub attrs: HMap<AAttrId, AAttrVal>,
    /// Refers effects of the item.
    pub effect_datas: HMap<AEffectId, AItemEffectData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<AEffectId>,
    /// Skill requirement map.
    pub srqs: HMap<AItemId, ASkillLevel>,
    /// Struct with extra data which is calculated during cache generation.
    pub extras: AItemExtras,
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
