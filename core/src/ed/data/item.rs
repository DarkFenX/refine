use crate::{
    defs::{AttrVal, ItemGrpId, ItemId},
    util::Named,
};

/// EVE item type data.
#[derive(Debug)]
pub struct EItem {
    /// Item type ID.
    pub id: ItemId,
    /// Refers an item group the item type belongs to.
    pub group_id: ItemGrpId,
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
        id: ItemId,
        group_id: ItemGrpId,
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
