use crate::{defines::ReeInt, dh};
use std::collections::{HashMap, HashSet};

/// Container for data, used internally by cache generator.
pub(in super::super) struct Data {
    pub(in super::super) items: Vec<dh::Item>,
    pub(in super::super) groups: Vec<dh::ItemGroup>,
    pub(in super::super) attrs: Vec<dh::Attr>,
    pub(in super::super) item_attrs: Vec<dh::ItemAttr>,
    pub(in super::super) effects: Vec<dh::Effect>,
    pub(in super::super) item_effects: Vec<dh::ItemEffect>,
    pub(in super::super) abils: Vec<dh::FighterAbil>,
    pub(in super::super) item_abils: Vec<dh::ItemFighterAbil>,
    pub(in super::super) buffs: Vec<dh::Buff>,
    pub(in super::super) item_srqs: Vec<dh::ItemSkillReq>,
    pub(in super::super) muta_items: Vec<dh::MutaItemConv>,
    pub(in super::super) muta_attrs: Vec<dh::MutaAttrMod>,
}
impl Data {
    pub(in super::super) fn new() -> Data {
        Data {
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

pub(in super::super) struct Support {
    pub(in super::super) attr_unit_map: HashMap<ReeInt, ReeInt>,
    pub(in super::super) grp_cat_map: HashMap<ReeInt, ReeInt>,
}
impl Support {
    pub(in super::super) fn new() -> Support {
        Support {
            attr_unit_map: HashMap::new(),
            grp_cat_map: HashMap::new(),
        }
    }
    pub(in super::super) fn post_pk(&mut self, data: &Data) {
        self.fill_attr_unit_map(&data);
        self.fill_grp_cat_map(&data);
    }
    fn fill_attr_unit_map(&mut self, data: &Data) {
        for attr in data.attrs.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
    fn fill_grp_cat_map(&mut self, data: &Data) {
        for grp in data.groups.iter() {
            self.grp_cat_map.insert(grp.id, grp.category_id);
        }
    }
}

pub(in super::super) struct KeyContainer {
    pub(in super::super) items: HashSet<ReeInt>,
    pub(in super::super) groups: HashSet<ReeInt>,
    pub(in super::super) attrs: HashSet<ReeInt>,
    pub(in super::super) effects: HashSet<ReeInt>,
    pub(in super::super) abils: HashSet<ReeInt>,
    pub(in super::super) buffs: HashSet<ReeInt>,
}
impl KeyContainer {
    pub(in super::super) fn new() -> KeyContainer {
        KeyContainer {
            items: HashSet::new(),
            groups: HashSet::new(),
            attrs: HashSet::new(),
            effects: HashSet::new(),
            abils: HashSet::new(),
            buffs: HashSet::new(),
        }
    }
}
