use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::{EAbilId, EAttrId, EBuffId, EData, EDataCont, EEffectId, EItemGrpId, EItemId, EItemListId},
    util::RSet,
};

pub(in crate::ad::generator) struct KeyDb {
    pub(in crate::ad::generator) items: RSet<EItemId>,
    pub(in crate::ad::generator) groups: RSet<EItemGrpId>,
    pub(in crate::ad::generator) item_lists: RSet<EItemListId>,
    pub(in crate::ad::generator) attrs: RSet<EAttrId>,
    pub(in crate::ad::generator) effects: RSet<EEffectId>,
    pub(in crate::ad::generator) abils: RSet<EAbilId>,
    pub(in crate::ad::generator) buffs: RSet<EBuffId>,
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
    fn extend_pk_vec<T: Pk>(set: &mut RSet<KeyPart>, cont: &EDataCont<T>) {
        for i in cont.data.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in crate::ad::generator) fn new_fkdb(e_data: &EData, g_supp: &GSupport) -> Self {
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(&e_data.items, g_supp);
        fkdb.extend_fk_vec(&e_data.groups, g_supp);
        fkdb.extend_fk_vec(&e_data.item_lists, g_supp);
        fkdb.extend_fk_vec(&e_data.attrs, g_supp);
        fkdb.extend_fk_vec(&e_data.item_attrs, g_supp);
        fkdb.extend_fk_vec(&e_data.effects, g_supp);
        fkdb.extend_fk_vec(&e_data.item_effects, g_supp);
        fkdb.extend_fk_vec(&e_data.abils, g_supp);
        fkdb.extend_fk_vec(&e_data.item_abils, g_supp);
        fkdb.extend_fk_vec(&e_data.buffs, g_supp);
        fkdb.extend_fk_vec(&e_data.space_comps, g_supp);
        fkdb.extend_fk_vec(&e_data.item_srqs, g_supp);
        fkdb.extend_fk_vec(&e_data.muta_items, g_supp);
        fkdb.extend_fk_vec(&e_data.muta_attrs, g_supp);
        fkdb.process_standalone_data(g_supp);
        fkdb
    }
    fn extend_fk_vec<T: Fk>(&mut self, cont: &EDataCont<T>, g_supp: &GSupport) {
        for v in cont.data.iter() {
            self.items.extend(v.get_item_fks(g_supp));
            self.groups.extend(v.get_group_fks(g_supp));
            self.item_lists.extend(v.get_item_list_fks(g_supp));
            self.attrs.extend(v.get_attr_fks(g_supp));
            self.effects.extend(v.get_effect_fks(g_supp));
            self.abils.extend(v.get_abil_fks(g_supp));
            self.buffs.extend(v.get_buff_fks(g_supp));
        }
    }
    fn process_standalone_data(&mut self, g_supp: &GSupport) {
        for a_attr in g_supp.standalone_attrs.iter() {
            self.attrs.extend(a_attr.iter_e_attr_ids());
        }
        for a_effect in g_supp.standalone_effects.iter() {
            self.items.extend(a_effect.iter_e_item_ids());
            self.attrs.extend(a_effect.iter_e_attr_ids());
            self.effects.extend(a_effect.iter_e_effect_ids());
            self.buffs.extend(a_effect.iter_e_buff_ids());
            self.item_lists.extend(a_effect.iter_e_item_list_ids());
        }
        for a_buff in g_supp.standalone_buffs.iter() {
            self.groups.extend(a_buff.iter_e_group_ids());
            self.attrs.extend(a_buff.iter_e_attr_ids());
            self.buffs.extend(a_buff.iter_e_buff_ids())
        }
    }
}
