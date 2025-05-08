use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed,
    util::RSet,
};

pub(in crate::adg) struct KeyDb {
    pub(in crate::adg) items: RSet<ed::EItemId>,
    pub(in crate::adg) groups: RSet<ed::EItemGrpId>,
    pub(in crate::adg) attrs: RSet<ed::EAttrId>,
    pub(in crate::adg) effects: RSet<ed::EEffectId>,
    pub(in crate::adg) abils: RSet<ed::EAbilId>,
    pub(in crate::adg) buffs: RSet<ed::EBuffId>,
}
impl KeyDb {
    pub(in crate::adg) fn new() -> Self {
        Self {
            items: RSet::new(),
            groups: RSet::new(),
            attrs: RSet::new(),
            effects: RSet::new(),
            abils: RSet::new(),
            buffs: RSet::new(),
        }
    }
    // Primary keys
    pub(in crate::adg) fn new_pkdb(e_data: &ed::EData) -> Self {
        let mut pkdb = Self::new();
        Self::extend_pk_vec(&mut pkdb.items, &e_data.items);
        Self::extend_pk_vec(&mut pkdb.groups, &e_data.groups);
        Self::extend_pk_vec(&mut pkdb.attrs, &e_data.attrs);
        Self::extend_pk_vec(&mut pkdb.effects, &e_data.effects);
        Self::extend_pk_vec(&mut pkdb.abils, &e_data.abils);
        Self::extend_pk_vec(&mut pkdb.buffs, &e_data.buffs);
        pkdb
    }
    fn extend_pk_vec<T: Pk>(set: &mut RSet<KeyPart>, cont: &ed::EDataCont<T>) {
        for i in cont.data.iter() {
            set.extend(i.get_pk())
        }
    }
    // Foreign keys
    pub(in crate::adg) fn new_fkdb(e_data: &ed::EData, g_supp: &GSupport) -> Self {
        let mut fkdb = Self::new();
        fkdb.extend_fk_vec(&e_data.items, g_supp);
        fkdb.extend_fk_vec(&e_data.groups, g_supp);
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
        fkdb
    }
    fn extend_fk_vec<T: Fk>(&mut self, cont: &ed::EDataCont<T>, g_supp: &GSupport) {
        for v in cont.data.iter() {
            self.items.extend(v.get_item_fks(g_supp));
            self.groups.extend(v.get_group_fks(g_supp));
            self.attrs.extend(v.get_attr_fks(g_supp));
            self.effects.extend(v.get_effect_fks(g_supp));
            self.abils.extend(v.get_abil_fks(g_supp));
            self.buffs.extend(v.get_buff_fks(g_supp));
        }
    }
}
