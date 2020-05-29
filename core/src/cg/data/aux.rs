use std::collections::{HashMap, HashSet};

use crate::{defines::ReeInt, dh};

use super::{Fk, Pk};

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

pub(in super::super) struct KeyDb {
    pub(in super::super) items: HashSet<ReeInt>,
    pub(in super::super) groups: HashSet<ReeInt>,
    pub(in super::super) attrs: HashSet<ReeInt>,
    pub(in super::super) effects: HashSet<ReeInt>,
    pub(in super::super) abils: HashSet<ReeInt>,
    pub(in super::super) buffs: HashSet<ReeInt>,
}
impl KeyDb {
    pub(in super::super) fn new() -> KeyDb {
        KeyDb {
            items: HashSet::new(),
            groups: HashSet::new(),
            attrs: HashSet::new(),
            effects: HashSet::new(),
            abils: HashSet::new(),
            buffs: HashSet::new(),
        }
    }
    // Primary keys
    pub(in super::super) fn new_pkdb(data: &Data) -> KeyDb {
        let mut pkdb = KeyDb::new();
        KeyDb::extend_pk_vec(&mut pkdb.items, &data.items);
        KeyDb::extend_pk_vec(&mut pkdb.groups, &data.groups);
        KeyDb::extend_pk_vec(&mut pkdb.attrs, &data.attrs);
        KeyDb::extend_pk_vec(&mut pkdb.effects, &data.effects);
        KeyDb::extend_pk_vec(&mut pkdb.abils, &data.abils);
        KeyDb::extend_pk_vec(&mut pkdb.buffs, &data.buffs);
        pkdb
    }
    fn extend_pk_vec<T: Pk>(set: &mut HashSet<ReeInt>, vec: &Vec<T>) {
        for i in vec.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in super::super) fn new_fkdb(data: &Data, supp: &Support) -> KeyDb {
        let mut fkdb = KeyDb::new();
        fkdb.extend_fk_vec(&data.items, &supp);
        fkdb.extend_fk_vec(&data.groups, &supp);
        fkdb.extend_fk_vec(&data.attrs, &supp);
        fkdb.extend_fk_vec(&data.item_attrs, &supp);
        fkdb.extend_fk_vec(&data.effects, &supp);
        fkdb.extend_fk_vec(&data.item_effects, &supp);
        fkdb.extend_fk_vec(&data.abils, &supp);
        fkdb.extend_fk_vec(&data.item_abils, &supp);
        fkdb.extend_fk_vec(&data.buffs, &supp);
        fkdb.extend_fk_vec(&data.item_srqs, &supp);
        fkdb.extend_fk_vec(&data.muta_items, &supp);
        fkdb.extend_fk_vec(&data.muta_attrs, &supp);
        fkdb
    }
    fn extend_fk_vec<T: Fk>(&mut self, vec: &Vec<T>, supp: &Support) {
        for v in vec.iter() {
            self.items.extend(v.get_item_fks(supp));
            self.groups.extend(v.get_group_fks(supp));
            self.attrs.extend(v.get_attr_fks(supp));
            self.effects.extend(v.get_effect_fks(supp));
            self.abils.extend(v.get_abil_fks(supp));
            self.buffs.extend(v.get_buff_fks(supp));
        }
    }
}
