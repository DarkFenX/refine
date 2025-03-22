use crate::{
    adg::{
        GData, GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed,
    util::StSet,
};

pub(in crate::adg) struct KeyDb {
    pub(in crate::adg) items: StSet<ed::EItemId>,
    pub(in crate::adg) groups: StSet<ed::EItemGrpId>,
    pub(in crate::adg) attrs: StSet<ed::EAttrId>,
    pub(in crate::adg) effects: StSet<ed::EEffectId>,
    pub(in crate::adg) abils: StSet<ed::EAbilId>,
    pub(in crate::adg) buffs: StSet<ed::EBuffId>,
}
impl KeyDb {
    pub(in crate::adg) fn new() -> Self {
        Self {
            items: StSet::new(),
            groups: StSet::new(),
            attrs: StSet::new(),
            effects: StSet::new(),
            abils: StSet::new(),
            buffs: StSet::new(),
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
    fn extend_pk_vec<T: Pk>(set: &mut StSet<KeyPart>, vec: &[T]) {
        for i in vec.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in crate::adg) fn new_fkdb(g_data: &GData, g_supp: &GSupport) -> Self {
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(&g_data.items, g_supp);
        fkdb.extend_fk_vec(&g_data.groups, g_supp);
        fkdb.extend_fk_vec(&g_data.attrs, g_supp);
        fkdb.extend_fk_vec(&g_data.item_attrs, g_supp);
        fkdb.extend_fk_vec(&g_data.effects, g_supp);
        fkdb.extend_fk_vec(&g_data.item_effects, g_supp);
        fkdb.extend_fk_vec(&g_data.abils, g_supp);
        fkdb.extend_fk_vec(&g_data.item_abils, g_supp);
        fkdb.extend_fk_vec(&g_data.buffs, g_supp);
        fkdb.extend_fk_vec(&g_data.item_srqs, g_supp);
        fkdb.extend_fk_vec(&g_data.muta_items, g_supp);
        fkdb.extend_fk_vec(&g_data.muta_attrs, g_supp);
        fkdb
    }
    fn extend_fk_vec<T: Fk>(&mut self, vec: &[T], g_supp: &GSupport) {
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
