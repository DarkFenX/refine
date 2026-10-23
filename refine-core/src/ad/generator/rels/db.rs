use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::{EData, EDataCont},
    util::RSet,
};

pub(in crate::ad::generator) struct KeyDb {
    pub(in crate::ad::generator) items: RSet<KeyPart>,
    pub(in crate::ad::generator) groups: RSet<KeyPart>,
    pub(in crate::ad::generator) item_lists: RSet<KeyPart>,
    pub(in crate::ad::generator) attrs: RSet<KeyPart>,
    pub(in crate::ad::generator) effects: RSet<KeyPart>,
    pub(in crate::ad::generator) abils: RSet<KeyPart>,
    pub(in crate::ad::generator) buffs: RSet<KeyPart>,
}
impl KeyDb {
    pub(in crate::ad::generator) fn new() -> Self {
        Self {
            items: RSet::new(),
            groups: RSet::new(),
            item_lists: RSet::new(),
            attrs: RSet::new(),
            effects: RSet::new(),
            abils: RSet::new(),
            buffs: RSet::new(),
        }
    }
    // Primary keys
    pub(in crate::ad::generator) fn new_pkdb(e_data: &EData) -> Self {
        let mut pkdb = Self::new();
        let Self {
            items,
            groups,
            item_lists,
            attrs,
            effects,
            abils,
            buffs,
        } = &mut pkdb;
        Self::extend_pk_vec(items, &e_data.items);
        Self::extend_pk_vec(groups, &e_data.groups);
        Self::extend_pk_vec(item_lists, &e_data.item_lists);
        Self::extend_pk_vec(attrs, &e_data.attrs);
        Self::extend_pk_vec(effects, &e_data.effects);
        Self::extend_pk_vec(abils, &e_data.abils);
        Self::extend_pk_vec(buffs, &e_data.buffs);
        pkdb
    }
    fn extend_pk_vec<T>(set: &mut RSet<KeyPart>, cont: &EDataCont<T>)
    where
        T: Pk,
    {
        for i in cont.data.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in crate::ad::generator) fn new_fkdb(e_data: &EData, adg_supp: &AdgSupport) -> Self {
        let EData {
            items,
            groups,
            item_lists,
            attrs,
            item_attrs,
            effects,
            item_effects,
            abils,
            item_abils,
            buffs,
            item_buffs,
            muta_items,
            muta_attrs,
        } = e_data;
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(items, adg_supp);
        fkdb.extend_fk_vec(groups, adg_supp);
        fkdb.extend_fk_vec(item_lists, adg_supp);
        fkdb.extend_fk_vec(attrs, adg_supp);
        fkdb.extend_fk_vec(item_attrs, adg_supp);
        fkdb.extend_fk_vec(effects, adg_supp);
        fkdb.extend_fk_vec(item_effects, adg_supp);
        fkdb.extend_fk_vec(abils, adg_supp);
        fkdb.extend_fk_vec(item_abils, adg_supp);
        fkdb.extend_fk_vec(buffs, adg_supp);
        fkdb.extend_fk_vec(item_buffs, adg_supp);
        fkdb.extend_fk_vec(muta_items, adg_supp);
        fkdb.extend_fk_vec(muta_attrs, adg_supp);
        fkdb.process_standalone_data(adg_supp);
        fkdb
    }
    fn extend_fk_vec<T>(&mut self, cont: &EDataCont<T>, adg_supp: &AdgSupport)
    where
        T: Fk,
    {
        let Self {
            items,
            groups,
            item_lists,
            attrs,
            effects,
            abils,
            buffs,
        } = self;
        for v in cont.data.iter() {
            items.extend(v.get_item_fks(adg_supp));
            groups.extend(v.get_group_fks(adg_supp));
            item_lists.extend(v.get_item_list_fks(adg_supp));
            attrs.extend(v.get_attr_fks(adg_supp));
            effects.extend(v.get_effect_fks(adg_supp));
            abils.extend(v.get_abil_fks(adg_supp));
            buffs.extend(v.get_buff_fks(adg_supp));
        }
    }
    fn process_standalone_data(&mut self, adg_supp: &AdgSupport) {
        let Self {
            items,
            groups,
            item_lists,
            attrs,
            effects,
            buffs,
            // Standalone data does not refer to abilities
            abils: _,
        } = self;
        for a_attr in adg_supp.standalone_attrs.iter() {
            attrs.extend(a_attr.iter_attr_eids().map(KeyPart::from_attr_eid));
        }
        for a_effect in adg_supp.standalone_effects.iter() {
            items.extend(a_effect.iter_item_eids().map(KeyPart::from_item_eid));
            attrs.extend(a_effect.iter_attr_eids().map(KeyPart::from_attr_eid));
            effects.extend(a_effect.iter_effect_eids().map(KeyPart::from_effect_eid));
            buffs.extend(a_effect.iter_buff_eids().map(KeyPart::from_buff_eid));
            item_lists.extend(a_effect.iter_item_list_eids().map(KeyPart::from_item_list_eid));
        }
        for a_buff in adg_supp.standalone_buffs.iter() {
            groups.extend(a_buff.iter_group_eids().map(KeyPart::from_item_grp_eid));
            attrs.extend(a_buff.iter_attr_eids().map(KeyPart::from_attr_eid));
            buffs.extend(a_buff.iter_buff_eids().map(KeyPart::from_buff_eid))
        }
    }
}
