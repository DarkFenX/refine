use crate::{
    ed::{EAttrVal, EItemGrpId, EItemId},
    util::Named,
};

/// EVE item type data.
pub struct EItem {
    /// Item type ID.
    pub id: EItemId,
    /// Refers an item group the item type belongs to.
    pub group_id: EItemGrpId,
    /// Base value of capacity attribute.
    pub capacity: EAttrVal,
    /// Base value of mass attribute.
    pub mass: EAttrVal,
    /// Base value of radius attribute.
    pub radius: EAttrVal,
    /// Base value of volume attribute.
    pub volume: EAttrVal,
}
impl Named for EItem {
    fn get_name() -> &'static str {
        "EItem"
    }
}
impl std::fmt::Display for EItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
