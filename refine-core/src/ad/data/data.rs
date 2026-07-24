use crate::ad::{AAbils, AAttrs, ABuffs, ADataWarnings, AEffects, AItemLists, AItems, AMutas};

#[cfg_attr(feature = "serde-ad", derive(serde::Serialize, serde::Deserialize))]
pub struct AData {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AItems::is_empty"))]
    pub items: AItems,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AAttrs::is_empty"))]
    pub attrs: AAttrs,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AMutas::is_empty"))]
    pub mutas: AMutas,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AEffects::is_empty"))]
    pub effects: AEffects,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ABuffs::is_empty"))]
    pub buffs: ABuffs,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AAbils::is_empty"))]
    pub abils: AAbils,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AItemLists::is_empty"))]
    pub item_lists: AItemLists,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ADataWarnings::is_empty"))]
    pub warnings: ADataWarnings,
}
impl AData {
    pub(in crate::ad) fn new() -> Self {
        Self {
            items: AItems::new(),
            attrs: AAttrs::new(),
            mutas: AMutas::new(),
            effects: AEffects::new(),
            buffs: ABuffs::new(),
            abils: AAbils::new(),
            item_lists: AItemLists::new(),
            warnings: ADataWarnings::new(),
        }
    }
}
