use crate::{
    ad::{
        AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemExtras, AItemGrpId, AItemId, AItemXt,
        ASkillLevel,
    },
    util::{Named, RMap},
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
    pub attrs: RMap<AAttrId, AAttrVal>,
    /// Refers effects of the item.
    pub effect_datas: RMap<AEffectId, AItemEffectData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<AEffectId>,
    /// Skill requirement map.
    pub srqs: RMap<AItemId, ASkillLevel>,
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

/// Adapted item with extra data added to it during runtime.
pub struct AItemRt {
    /// Adapted item.
    pub ai: AItem,
    /// Extra data, which is generated using data from adapted data during runtime.
    pub(crate) xt: AItemXt,
}
impl AItemRt {
    /// Construct new adapted item with extra data.
    pub fn new(a_item: AItem) -> Self {
        Self {
            ai: a_item,
            xt: AItemXt {},
        }
    }
}
impl Named for AItemRt {
    fn get_name() -> &'static str {
        "AItemRt"
    }
}
impl std::fmt::Display for AItemRt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.ai.id)
    }
}
