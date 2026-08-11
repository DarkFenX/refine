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
        Self::extend_pk_vec(&mut pkdb.items, &e_data.items);
        Self::extend_pk_vec(&mut pkdb.groups, &e_data.groups);
        Self::extend_pk_vec(&mut pkdb.item_lists, &e_data.item_lists);
        Self::extend_pk_vec(&mut pkdb.attrs, &e_data.attrs);
        Self::extend_pk_vec(&mut pkdb.effects, &e_data.effects);
        Self::extend_pk_vec(&mut pkdb.abils, &e_data.abils);
        Self::extend_pk_vec(&mut pkdb.buffs, &e_data.buffs);
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
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(&e_data.items, adg_supp);
        fkdb.extend_fk_vec(&e_data.groups, adg_supp);
        fkdb.extend_fk_vec(&e_data.item_lists, adg_supp);
        fkdb.extend_fk_vec(&e_data.attrs, adg_supp);
        fkdb.extend_fk_vec(&e_data.item_attrs, adg_supp);
        fkdb.extend_fk_vec(&e_data.effects, adg_supp);
        fkdb.extend_fk_vec(&e_data.item_effects, adg_supp);
        fkdb.extend_fk_vec(&e_data.abils, adg_supp);
        fkdb.extend_fk_vec(&e_data.item_abils, adg_supp);
        fkdb.extend_fk_vec(&e_data.buffs, adg_supp);
        fkdb.extend_fk_vec(&e_data.space_comps, adg_supp);
        fkdb.extend_fk_vec(&e_data.item_srqs, adg_supp);
        fkdb.extend_fk_vec(&e_data.muta_items, adg_supp);
        fkdb.extend_fk_vec(&e_data.muta_attrs, adg_supp);
        fkdb.process_standalone_data(adg_supp);
        fkdb
    }
    fn extend_fk_vec<T>(&mut self, cont: &EDataCont<T>, adg_supp: &AdgSupport)
    where
        T: Fk,
    {
        for v in cont.data.iter() {
            self.items.extend(v.get_item_fks(adg_supp));
            self.groups.extend(v.get_group_fks(adg_supp));
            self.item_lists.extend(v.get_item_list_fks(adg_supp));
            self.attrs.extend(v.get_attr_fks(adg_supp));
            self.effects.extend(v.get_effect_fks(adg_supp));
            self.abils.extend(v.get_abil_fks(adg_supp));
            self.buffs.extend(v.get_buff_fks(adg_supp));
        }
    }
    fn process_standalone_data(&mut self, adg_supp: &AdgSupport) {
        for a_attr in adg_supp.standalone_attrs.iter() {
            self.attrs.extend(a_attr.iter_attr_eids().map(KeyPart::from_attr_eid));
        }
        for a_effect in adg_supp.standalone_effects.iter() {
            self.items.extend(a_effect.iter_item_eids().map(KeyPart::from_item_eid));
            self.attrs.extend(a_effect.iter_attr_eids().map(KeyPart::from_attr_eid));
            self.effects
                .extend(a_effect.iter_effect_eids().map(KeyPart::from_effect_eid));
            self.buffs.extend(a_effect.iter_buff_eids().map(KeyPart::from_buff_eid));
            self.item_lists
                .extend(a_effect.iter_item_list_eids().map(KeyPart::from_item_list_eid));
        }
        for a_buff in adg_supp.standalone_buffs.iter() {
            self.groups
                .extend(a_buff.iter_group_eids().map(KeyPart::from_item_grp_eid));
            self.attrs.extend(a_buff.iter_attr_eids().map(KeyPart::from_attr_eid));
            self.buffs.extend(a_buff.iter_buff_eids().map(KeyPart::from_buff_eid))
        }
    }
}
