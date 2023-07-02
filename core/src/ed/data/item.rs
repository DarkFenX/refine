use crate::{
    defs::{AttrVal, EItemGrpId, EItemId},
    util::Named,
};

/// EVE item type data.
pub struct EItem {
    /// Item type ID.
    pub id: EItemId,
    /// Refers an item group the item type belongs to.
    pub group_id: EItemGrpId,
    /// Base value of capacity attribute.
    pub capacity: AttrVal,
    /// Base value of mass attribute.
    pub mass: AttrVal,
    /// Base value of radius attribute.
    pub radius: AttrVal,
    /// Base value of volume attribute.
    pub volume: AttrVal,
}
impl EItem {
    /// Make a new EVE item type out of passed data.
    pub fn new(
        id: EItemId,
        group_id: EItemGrpId,
        capacity: AttrVal,
        mass: AttrVal,
        radius: AttrVal,
        volume: AttrVal,
    ) -> Self {
        Self {
            id,
            group_id,
            capacity,
            mass,
            radius,
            volume,
        }
    }
}
impl Named for EItem {
    fn get_name() -> &'static str {
        "EItem"
    }
}
