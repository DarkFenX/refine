use std::collections::HashSet;

use crate::{
    adg::{
        rels::{Fk, Pk},
        GData, GSupport,
    },
    defs::ReeInt,
};

pub(in crate::adg) struct KeyDb {
    pub(in crate::adg) items: HashSet<ReeInt>,
    pub(in crate::adg) groups: HashSet<ReeInt>,
    pub(in crate::adg) attrs: HashSet<ReeInt>,
    pub(in crate::adg) effects: HashSet<ReeInt>,
    pub(in crate::adg) abils: HashSet<ReeInt>,
    pub(in crate::adg) buffs: HashSet<ReeInt>,
}
impl KeyDb {
    pub(in crate::adg) fn new() -> Self {
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
    pub(in crate::adg) fn new_pkdb(gdata: &GData) -> Self {
        let mut pkdb = Self::new();
        Self::extend_pk_vec(&mut pkdb.items, &gdata.items);
        Self::extend_pk_vec(&mut pkdb.groups, &gdata.groups);
        Self::extend_pk_vec(&mut pkdb.attrs, &gdata.attrs);
        Self::extend_pk_vec(&mut pkdb.effects, &gdata.effects);
        Self::extend_pk_vec(&mut pkdb.abils, &gdata.abils);
        Self::extend_pk_vec(&mut pkdb.buffs, &gdata.buffs);
        pkdb
    }
    fn extend_pk_vec<T: Pk>(set: &mut HashSet<ReeInt>, vec: &Vec<T>) {
        for i in vec.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in crate::adg) fn new_fkdb(gdata: &GData, gsupp: &GSupport) -> Self {
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(&gdata.items, &gsupp);
        fkdb.extend_fk_vec(&gdata.groups, &gsupp);
        fkdb.extend_fk_vec(&gdata.attrs, &gsupp);
        fkdb.extend_fk_vec(&gdata.item_attrs, &gsupp);
        fkdb.extend_fk_vec(&gdata.effects, &gsupp);
        fkdb.extend_fk_vec(&gdata.item_effects, &gsupp);
        fkdb.extend_fk_vec(&gdata.abils, &gsupp);
        fkdb.extend_fk_vec(&gdata.item_abils, &gsupp);
        fkdb.extend_fk_vec(&gdata.buffs, &gsupp);
        fkdb.extend_fk_vec(&gdata.item_srqs, &gsupp);
        fkdb.extend_fk_vec(&gdata.muta_items, &gsupp);
        fkdb.extend_fk_vec(&gdata.muta_attrs, &gsupp);
        fkdb
    }
    fn extend_fk_vec<T: Fk>(&mut self, vec: &Vec<T>, gsupp: &GSupport) {
        for v in vec.iter() {
            self.items.extend(v.get_item_fks(gsupp));
            self.groups.extend(v.get_group_fks(gsupp));
            self.attrs.extend(v.get_attr_fks(gsupp));
            self.effects.extend(v.get_effect_fks(gsupp));
            self.abils.extend(v.get_abil_fks(gsupp));
            self.buffs.extend(v.get_buff_fks(gsupp));
        }
    }
}
