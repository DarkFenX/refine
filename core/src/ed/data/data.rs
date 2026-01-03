use crate::ed::data::{
    EAbil, EAttr, EBuff, EEffect, EItem, EItemAbil, EItemAttr, EItemEffect, EItemGroup, EItemList, EItemSkillReq,
    EItemSpaceComp, EMutaAttrMod, EMutaItemConv,
};

pub struct EData {
    pub items: EDataCont<EItem>,
    pub groups: EDataCont<EItemGroup>,
    pub item_lists: EDataCont<EItemList>,
    pub attrs: EDataCont<EAttr>,
    pub item_attrs: EDataCont<EItemAttr>,
    pub effects: EDataCont<EEffect>,
    pub item_effects: EDataCont<EItemEffect>,
    pub abils: EDataCont<EAbil>,
    pub item_abils: EDataCont<EItemAbil>,
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

pub struct EDataCont<T> {
    pub data: Vec<T>,
    pub warns: Vec<String>,
}
impl<T> EDataCont<T> {
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
