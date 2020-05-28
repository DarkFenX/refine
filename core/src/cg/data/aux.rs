use crate::dh;

/// Container for data, used internally by cache generator.
pub(in super::super) struct Data {
    pub(in super::super) items: Vec<dh::Item>,
    pub(in super::super) item_groups: Vec<dh::ItemGroup>,
    pub(in super::super) attrs: Vec<dh::Attr>,
    pub(in super::super) item_attrs: Vec<dh::ItemAttr>,
    pub(in super::super) effects: Vec<dh::Effect>,
    pub(in super::super) item_effects: Vec<dh::ItemEffect>,
    pub(in super::super) fighter_abils: Vec<dh::FighterAbil>,
    pub(in super::super) item_fighter_abils: Vec<dh::ItemFighterAbil>,
    pub(in super::super) buffs: Vec<dh::Buff>,
    pub(in super::super) item_skill_reqs: Vec<dh::ItemSkillReq>,
    pub(in super::super) muta_item_convs: Vec<dh::MutaItemConv>,
    pub(in super::super) muta_attr_mods: Vec<dh::MutaAttrMod>,
}
impl Data {
    pub(in super::super) fn new() -> Data {
        Data {
            items: Vec::new(),
            item_groups: Vec::new(),
            attrs: Vec::new(),
            item_attrs: Vec::new(),
            effects: Vec::new(),
            item_effects: Vec::new(),
            fighter_abils: Vec::new(),
            item_fighter_abils: Vec::new(),
            buffs: Vec::new(),
            item_skill_reqs: Vec::new(),
            muta_item_convs: Vec::new(),
            muta_attr_mods: Vec::new(),
        }
    }
}
