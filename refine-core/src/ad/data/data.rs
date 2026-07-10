use crate::ad::{AAbils, AAttrs, ABuffs, ADataWarnings, AEffects, AItemLists, AItems, AMutas};

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
