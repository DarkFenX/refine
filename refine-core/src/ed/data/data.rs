use crate::ed::data::{
    EAbil, EAttr, EBuff, EEffect, EItem, EItemAbil, EItemAttr, EItemBuff, EItemEffect, EItemGroup, EItemList,
    EMutaAttr, EMutaItem,
};

#[derive(Default)]
pub struct EData {
    pub items: EDataCont<EItem> = EDataCont::default(),
    pub groups: EDataCont<EItemGroup> = EDataCont::default(),
    pub item_lists: EDataCont<EItemList> = EDataCont::default(),
    pub attrs: EDataCont<EAttr> = EDataCont::default(),
    pub item_attrs: EDataCont<EItemAttr> = EDataCont::default(),
    pub effects: EDataCont<EEffect> = EDataCont::default(),
    pub item_effects: EDataCont<EItemEffect> = EDataCont::default(),
    pub abils: EDataCont<EAbil> = EDataCont::default(),
    pub item_abils: EDataCont<EItemAbil> = EDataCont::default(),
    pub buffs: EDataCont<EBuff> = EDataCont::default(),
    pub item_buffs: EDataCont<EItemBuff> = EDataCont::default(),
    pub muta_items: EDataCont<EMutaItem> = EDataCont::default(),
    pub muta_attrs: EDataCont<EMutaAttr> = EDataCont::default(),
}
impl EData {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct EDataCont<T> {
    pub data: Vec<T>,
    pub warnings: Vec<String>,
}
impl<T> EDataCont<T> {
    pub fn new() -> EDataCont<T> {
        Self::default()
    }
    pub fn with_capacity(capacity: usize) -> EDataCont<T> {
        EDataCont {
            data: Vec::with_capacity(capacity),
            warnings: Vec::new(),
        }
    }
}
const impl<T> Default for EDataCont<T> {
    fn default() -> Self {
        EDataCont {
            data: Vec::new(),
            warnings: Vec::new(),
        }
    }
}
