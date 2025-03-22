use crate::ed;

/// Container for primary data, used internally by the generator.
pub(in crate::adg) struct GData {
    pub(in crate::adg) items: Vec<ed::EItem>,
    pub(in crate::adg) groups: Vec<ed::EItemGroup>,
    pub(in crate::adg) attrs: Vec<ed::EAttr>,
    pub(in crate::adg) item_attrs: Vec<ed::EItemAttr>,
    pub(in crate::adg) effects: Vec<ed::EEffect>,
    pub(in crate::adg) item_effects: Vec<ed::EItemEffect>,
    pub(in crate::adg) abils: Vec<ed::EFighterAbil>,
    pub(in crate::adg) item_abils: Vec<ed::EItemFighterAbil>,
    pub(in crate::adg) buffs: Vec<ed::EBuff>,
    pub(in crate::adg) item_srqs: Vec<ed::EItemSkillReq>,
    pub(in crate::adg) muta_items: Vec<ed::EMutaItemConv>,
    pub(in crate::adg) muta_attrs: Vec<ed::EMutaAttrMod>,
}
impl GData {
    pub(in crate::adg) fn new() -> Self {
        Self {
            items: Vec::new(),
            groups: Vec::new(),
            attrs: Vec::new(),
            item_attrs: Vec::new(),
            effects: Vec::new(),
            item_effects: Vec::new(),
            abils: Vec::new(),
            item_abils: Vec::new(),
            buffs: Vec::new(),
            item_srqs: Vec::new(),
            muta_items: Vec::new(),
            muta_attrs: Vec::new(),
        }
    }
}
