use crate::dh;

/// Container for data, used internally by cache generator.
pub(super) struct Data {
    pub(super) items: Vec<dh::Item>,
    pub(super) item_groups: Vec<dh::ItemGroup>,
    pub(super) attrs: Vec<dh::Attr>,
    pub(super) item_attrs: Vec<dh::ItemAttr>,
    pub(super) effects: Vec<dh::Effect>,
    pub(super) item_effects: Vec<dh::ItemEffect>,
    pub(super) fighter_abils: Vec<dh::FighterAbil>,
    pub(super) item_fighter_abils: Vec<dh::ItemFighterAbil>,
    pub(super) buffs: Vec<dh::Buff>,
    pub(super) item_skill_reqs: Vec<dh::ItemSkillReq>,
    pub(super) muta_item_convs: Vec<dh::MutaItemConv>,
    pub(super) muta_attr_mods: Vec<dh::MutaAttrMod>,
}
impl Data {
    pub(super) fn new() -> Data {
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
