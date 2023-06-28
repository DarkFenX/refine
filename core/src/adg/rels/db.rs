use std::collections::HashSet;

use crate::{
    adg::{
        rels::{Fk, KeyPart, Pk},
        GData, GSupport,
    },
    defs::{AbilId, AttrId, BuffId, EffectId, ItemGrpId, ItemId},
};

pub(in crate::adg) struct KeyDb {
    pub(in crate::adg) items: HashSet<ItemId>,
    pub(in crate::adg) groups: HashSet<ItemGrpId>,
    pub(in crate::adg) attrs: HashSet<AttrId>,
    pub(in crate::adg) effects: HashSet<EffectId>,
    pub(in crate::adg) abils: HashSet<AbilId>,
    pub(in crate::adg) buffs: HashSet<BuffId>,
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
    pub(in crate::adg) fn new_pkdb(g_data: &GData) -> Self {
        let mut pkdb = Self::new();
        Self::extend_pk_vec(&mut pkdb.items, &g_data.items);
        Self::extend_pk_vec(&mut pkdb.groups, &g_data.groups);
        Self::extend_pk_vec(&mut pkdb.attrs, &g_data.attrs);
        Self::extend_pk_vec(&mut pkdb.effects, &g_data.effects);
        Self::extend_pk_vec(&mut pkdb.abils, &g_data.abils);
        Self::extend_pk_vec(&mut pkdb.buffs, &g_data.buffs);
        pkdb
    }
    fn extend_pk_vec<T: Pk>(set: &mut HashSet<KeyPart>, vec: &Vec<T>) {
        for i in vec.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in crate::adg) fn new_fkdb(g_data: &GData, g_supp: &GSupport) -> Self {
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(&g_data.items, &g_supp);
        fkdb.extend_fk_vec(&g_data.groups, &g_supp);
        fkdb.extend_fk_vec(&g_data.attrs, &g_supp);
        fkdb.extend_fk_vec(&g_data.item_attrs, &g_supp);
        fkdb.extend_fk_vec(&g_data.effects, &g_supp);
        fkdb.extend_fk_vec(&g_data.item_effects, &g_supp);
        fkdb.extend_fk_vec(&g_data.abils, &g_supp);
        fkdb.extend_fk_vec(&g_data.item_abils, &g_supp);
        fkdb.extend_fk_vec(&g_data.buffs, &g_supp);
        fkdb.extend_fk_vec(&g_data.item_srqs, &g_supp);
        fkdb.extend_fk_vec(&g_data.muta_items, &g_supp);
        fkdb.extend_fk_vec(&g_data.muta_attrs, &g_supp);
        fkdb
    }
    fn extend_fk_vec<T: Fk>(&mut self, vec: &Vec<T>, g_supp: &GSupport) {
        for v in vec.iter() {
            self.items.extend(v.get_item_fks(g_supp));
            self.groups.extend(v.get_group_fks(g_supp));
            self.attrs.extend(v.get_attr_fks(g_supp));
            self.effects.extend(v.get_effect_fks(g_supp));
            self.abils.extend(v.get_abil_fks(g_supp));
            self.buffs.extend(v.get_buff_fks(g_supp));
        }
    }
}
