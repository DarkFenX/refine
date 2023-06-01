use std::collections::{HashMap, HashSet};

use crate::{
    defs::{ReeFloat, ReeInt},
    edt,
};

use super::{Fk, Pk};

// Since CCP data is full of dead references to various entities with value 0, I assume it stands
// for "no reference"
pub(super) fn attrval_to_fk(val: Option<ReeFloat>) -> Option<ReeInt> {
    val.and_then(|v| if v == 0.0 { None } else { Some(v.round() as ReeInt) })
}

/// Container for data, used internally by cache generator.
pub(in super::super) struct Data {
    pub(in super::super) items: Vec<edt::Item>,
    pub(in super::super) groups: Vec<edt::ItemGroup>,
    pub(in super::super) attrs: Vec<edt::Attr>,
    pub(in super::super) item_attrs: Vec<edt::ItemAttr>,
    pub(in super::super) effects: Vec<edt::Effect>,
    pub(in super::super) item_effects: Vec<edt::ItemEffect>,
    pub(in super::super) abils: Vec<edt::FighterAbil>,
    pub(in super::super) item_abils: Vec<edt::ItemFighterAbil>,
    pub(in super::super) buffs: Vec<edt::Buff>,
    pub(in super::super) item_srqs: Vec<edt::ItemSkillReq>,
    pub(in super::super) muta_items: Vec<edt::MutaItemConv>,
    pub(in super::super) muta_attrs: Vec<edt::MutaAttrMod>,
}
impl Data {
    pub(in super::super) fn new() -> Self {
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

pub(in super::super) struct Support {
    pub(in super::super) attr_unit_map: HashMap<ReeInt, ReeInt>,
    pub(in super::super) grp_cat_map: HashMap<ReeInt, ReeInt>,
}
impl Support {
    pub(in super::super) fn new() -> Self {
        Self {
            attr_unit_map: HashMap::new(),
            grp_cat_map: HashMap::new(),
        }
    }
    pub(in super::super) fn post_pk(&mut self, erg_data: &Data) {
        self.fill_attr_unit_map(&erg_data);
        self.fill_grp_cat_map(&erg_data);
    }
    fn fill_attr_unit_map(&mut self, erg_data: &Data) {
        for attr in erg_data.attrs.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
    fn fill_grp_cat_map(&mut self, erg_data: &Data) {
        for grp in erg_data.groups.iter() {
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
    pub(in super::super) fn new() -> Self {
        Self {
            items: HashSet::new(),
            groups: HashSet::new(),
            attrs: HashSet::new(),
            effects: HashSet::new(),
            abils: HashSet::new(),
            buffs: HashSet::new(),
        }
    }
    // Primary keys
    pub(in super::super) fn new_pkdb(erg_data: &Data) -> Self {
        let mut pkdb = Self::new();
        Self::extend_pk_vec(&mut pkdb.items, &erg_data.items);
        Self::extend_pk_vec(&mut pkdb.groups, &erg_data.groups);
        Self::extend_pk_vec(&mut pkdb.attrs, &erg_data.attrs);
        Self::extend_pk_vec(&mut pkdb.effects, &erg_data.effects);
        Self::extend_pk_vec(&mut pkdb.abils, &erg_data.abils);
        Self::extend_pk_vec(&mut pkdb.buffs, &erg_data.buffs);
        pkdb
    }
    fn extend_pk_vec<T: Pk>(set: &mut HashSet<ReeInt>, vec: &Vec<T>) {
        for i in vec.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in super::super) fn new_fkdb(erg_data: &Data, supp: &Support) -> Self {
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(&erg_data.items, &supp);
        fkdb.extend_fk_vec(&erg_data.groups, &supp);
        fkdb.extend_fk_vec(&erg_data.attrs, &supp);
        fkdb.extend_fk_vec(&erg_data.item_attrs, &supp);
        fkdb.extend_fk_vec(&erg_data.effects, &supp);
        fkdb.extend_fk_vec(&erg_data.item_effects, &supp);
        fkdb.extend_fk_vec(&erg_data.abils, &supp);
        fkdb.extend_fk_vec(&erg_data.item_abils, &supp);
        fkdb.extend_fk_vec(&erg_data.buffs, &supp);
        fkdb.extend_fk_vec(&erg_data.item_srqs, &supp);
        fkdb.extend_fk_vec(&erg_data.muta_items, &supp);
        fkdb.extend_fk_vec(&erg_data.muta_attrs, &supp);
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
