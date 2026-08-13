use crate::ad::{AAbils, AAttrs, ABuffs, ADataWarnings, AEffects, AItemLists, AItems, AMutas};

#[cfg_attr(feature = "serde-ad", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default)]
pub struct AData {
    pub items: AItems,
    pub attrs: AAttrs,
    pub mutas: AMutas,
    pub effects: AEffects,
    pub buffs: ABuffs,
    pub abils: AAbils,
    pub item_lists: AItemLists,
    pub warnings: ADataWarnings,
}
impl AData {
    pub fn new() -> Self {
        Self::default()
    }
}
