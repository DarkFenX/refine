use crate::ed::data::{
    EAttr, EBuff, EEffect, EFighterAbil, EItem, EItemAttr, EItemEffect, EItemFighterAbil, EItemGroup, EItemList,
    EItemSkillReq, EItemSpaceComp, EMutaAttrMod, EMutaItemConv,
};

/// Container for primary EVE data.
pub struct EData {
    pub items: EDataCont<EItem>,
    pub groups: EDataCont<EItemGroup>,
    pub item_lists: EDataCont<EItemList>,
    pub attrs: EDataCont<EAttr>,
    pub item_attrs: EDataCont<EItemAttr>,
    pub effects: EDataCont<EEffect>,
    pub item_effects: EDataCont<EItemEffect>,
    pub abils: EDataCont<EFighterAbil>,
    pub item_abils: EDataCont<EItemFighterAbil>,
    pub buffs: EDataCont<EBuff>,
    pub space_comps: EDataCont<EItemSpaceComp>,
    pub item_srqs: EDataCont<EItemSkillReq>,
    pub muta_items: EDataCont<EMutaItemConv>,
    pub muta_attrs: EDataCont<EMutaAttrMod>,
}
impl EData {
    pub fn new() -> Self {
        Self {
            items: EDataCont::new(),
            groups: EDataCont::new(),
            item_lists: EDataCont::new(),
            attrs: EDataCont::new(),
            item_attrs: EDataCont::new(),
            effects: EDataCont::new(),
            item_effects: EDataCont::new(),
            abils: EDataCont::new(),
            item_abils: EDataCont::new(),
            buffs: EDataCont::new(),
            space_comps: EDataCont::new(),
            item_srqs: EDataCont::new(),
            muta_items: EDataCont::new(),
            muta_attrs: EDataCont::new(),
        }
    }
}
impl Default for EData {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience type to pass data and accumulated warnings to the caller.
pub struct EDataCont<T> {
    /// Vector with actual data.
    pub data: Vec<T>,
    /// Vector with strings which represent warnings encountered during data retrieval.
    pub warns: Vec<String>,
}
impl<T> EDataCont<T> {
    /// Make a new empty container.
    pub fn new() -> EDataCont<T> {
        EDataCont {
            data: Vec::new(),
            warns: Vec::new(),
        }
    }
}
impl<T> Default for EDataCont<T> {
    fn default() -> Self {
        Self::new()
    }
}
