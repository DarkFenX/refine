use crate::{
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// EVE item type data.
#[derive(Debug)]
pub struct EItem {
    /// Item type ID.
    pub id: ReeInt,
    /// Refers an item group the item type belongs to.
    pub group_id: ReeInt,
    /// Base value of capacity attribute.
    pub capacity: ReeFloat,
    /// Base value of mass attribute.
    pub mass: ReeFloat,
    /// Base value of radius attribute.
    pub radius: ReeFloat,
    /// Base value of volume attribute.
    pub volume: ReeFloat,
}
impl EItem {
    /// Make a new EVE item type out of passed data.
    pub fn new(
        id: ReeInt,
        group_id: ReeInt,
        capacity: ReeFloat,
        mass: ReeFloat,
        radius: ReeFloat,
        volume: ReeFloat,
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
