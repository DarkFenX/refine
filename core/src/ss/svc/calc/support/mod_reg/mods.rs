use std::hash::Hash;

use crate::{
    defs::{EItemGrpId, EItemId, SsFitId, SsItemId},
    shr::ModDomain,
    ss::{
        fit::{SsFit, SsFits},
        item::{SsItem, SsItems},
    },
    util::KeyedStorage1L,
    EAttrId,
};

use super::{
    super::modifier::{SsAttrMod, SsModTgtFilter},
    DomsAct,
};

pub(super) struct ModifierRegister {
    // Modifiers registered for an item
    // Contains: KeyedStorage<modifier item ID, modifiers>
    mods: KeyedStorage1L<SsItemId, SsAttrMod>,
    // Modifiers which modify item directly
    // Contains: KeyedStorage<modifier item ID, modifiers>
    mods_direct: KeyedStorage1L<SsItemId, SsAttrMod>,
    // All modifiers which modify top-level entities (via ship or character reference) are kept here
    // Contains: KeyedStorage<(target's fit ID, target's domain), modifiers>
    mods_topdom: KeyedStorage1L<(SsFitId, ModDomain), SsAttrMod>,
    // Modifiers which modify 'other' location are always stored here, regardless if they actually
    // modify something or not
    // Contains: KeyedStorage<modifier item ID, modifiers>
    mods_other: KeyedStorage1L<SsItemId, SsAttrMod>,
    // Modifiers influencing all items belonging to certain fit and domain
    // Contains: KeyedStorage<(target's fit ID, target's domain), modifiers>
    mods_pardom: KeyedStorage1L<(SsFitId, ModDomain), SsAttrMod>,
    // Modifiers influencing items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(target's fit ID, target's domain, target's group ID), modifiers>
    mods_pardom_grp: KeyedStorage1L<(SsFitId, ModDomain, EItemGrpId), SsAttrMod>,
    // Modifiers influencing items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's domain, target's skillreq type ID), modifiers>
    mods_pardom_srq: KeyedStorage1L<(SsFitId, ModDomain, EItemId), SsAttrMod>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), modifiers>
    mods_own_srq: KeyedStorage1L<(SsFitId, EItemId), SsAttrMod>,
}
impl ModifierRegister {
    pub(super) fn new() -> Self {
        Self {
            mods: KeyedStorage1L::new(),
            mods_direct: KeyedStorage1L::new(),
            mods_topdom: KeyedStorage1L::new(),
            mods_other: KeyedStorage1L::new(),
            mods_pardom: KeyedStorage1L::new(),
            mods_pardom_grp: KeyedStorage1L::new(),
            mods_pardom_srq: KeyedStorage1L::new(),
            mods_own_srq: KeyedStorage1L::new(),
        }
    }
    // Query methods
    pub(super) fn get_mods_for_tgt(&self, tgt_item: &SsItem, tgt_attr_id: &EAttrId, fits: &SsFits) -> Vec<SsAttrMod> {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_topdom_opt = tgt_item.get_top_domain();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        let mut mods = Vec::new();
        filter_and_extend(&mut mods, &self.mods_direct, &tgt_item_id, tgt_attr_id);
        if let (Some(tgt_fit), Some(tgt_topdom)) = (tgt_fit_opt, tgt_topdom_opt) {
            filter_and_extend(&mut mods, &self.mods_topdom, &(tgt_fit.id, tgt_topdom), tgt_attr_id);
        }
        if let Some(other_item_id) = tgt_item.get_other() {
            filter_and_extend(&mut mods, &self.mods_other, &other_item_id, tgt_attr_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for dom in DomsAct::new(tgt_item, tgt_fit) {
                filter_and_extend(&mut mods, &self.mods_pardom, &(tgt_fit.id, dom), tgt_attr_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for dom in DomsAct::new(tgt_item, tgt_fit) {
                filter_and_extend(
                    &mut mods,
                    &self.mods_pardom_grp,
                    &(tgt_fit.id, dom, tgt_grp_id),
                    tgt_attr_id,
                );
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for dom in DomsAct::new(tgt_item, tgt_fit) {
                for skill_a_item_id in tgt_srqs.keys() {
                    filter_and_extend(
                        &mut mods,
                        &self.mods_pardom_srq,
                        &(tgt_fit.id, dom, *skill_a_item_id),
                        tgt_attr_id,
                    );
                }
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    filter_and_extend(
                        &mut mods,
                        &self.mods_own_srq,
                        &(tgt_fit.id, *skill_a_item_id),
                        tgt_attr_id,
                    );
                }
            }
        }
        mods
    }
    pub(super) fn get_mods_for_changed_domain_owner(&mut self, item: &SsItem, items: &SsItems) -> Vec<SsAttrMod> {
        let mut mods = Vec::new();
        if let (Some(fit_id), Some(dom)) = (item.get_fit_id(), item.get_top_domain()) {
            for (sub_item_id, sub_mods) in self.mods.iter() {
                if let Ok(sub_item) = items.get_item(sub_item_id) {
                    if sub_item.get_fit_id() == Some(fit_id) {
                        for sub_mod in sub_mods.iter() {
                            if match sub_mod.tgt_filter {
                                SsModTgtFilter::Loc(sub_dom) => dom == sub_dom,
                                SsModTgtFilter::LocGrp(sub_dom, _) => dom == sub_dom,
                                SsModTgtFilter::LocSrq(sub_dom, _) => dom == sub_dom,
                                _ => false,
                            } {
                                mods.push(*sub_mod);
                            }
                        }
                    }
                }
            }
        }
        mods
    }
    pub(super) fn iter_mods_for_src(&self, src_item_id: &SsItemId) -> impl Iterator<Item = &SsAttrMod> {
        self.mods.get(src_item_id).into_iter().flatten()
    }
    // Modification methods
    pub(super) fn reg_mod(&mut self, src_fit_opt: Option<&SsFit>, modifier: SsAttrMod) {
        self.mods.add(modifier.src_item_id, modifier);
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                ModDomain::Item => self.mods_direct.add(modifier.src_item_id, modifier),
                ModDomain::Char | ModDomain::Ship | ModDomain::Structure => {
                    if let Some(src_fit) = src_fit_opt {
                        self.mods_topdom.add((src_fit.id, dom), modifier);
                    }
                }
                ModDomain::Other => self.mods_other.add(modifier.src_item_id, modifier),
            },
            SsModTgtFilter::Loc(dom) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_pardom.add((src_fit.id, dom), modifier);
                }
            }
            SsModTgtFilter::LocGrp(dom, grp_id) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_pardom_grp.add((src_fit.id, dom, grp_id), modifier);
                }
            }
            SsModTgtFilter::LocSrq(dom, srq_id) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_pardom_srq.add((src_fit.id, dom, srq_id), modifier);
                }
            }
            SsModTgtFilter::OwnSrq(srq_id) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_own_srq.add((src_fit.id, srq_id), modifier);
                }
            }
        }
    }
    pub(super) fn unreg_mod(&mut self, src_fit_opt: Option<&SsFit>, modifier: &SsAttrMod) {
        self.mods.remove(&modifier.src_item_id, &modifier);
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                ModDomain::Item => self.mods_direct.remove(&modifier.src_item_id, &modifier),
                ModDomain::Char | ModDomain::Ship | ModDomain::Structure => {
                    if let Some(src_fit) = src_fit_opt {
                        self.mods_topdom.remove(&(src_fit.id, dom), &modifier);
                    }
                }
                ModDomain::Other => self.mods_other.remove(&modifier.src_item_id, &modifier),
            },
            SsModTgtFilter::Loc(dom) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_pardom.remove(&(src_fit.id, dom), &modifier);
                }
            }
            SsModTgtFilter::LocGrp(dom, grp_id) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_pardom_grp.remove(&(src_fit.id, dom, grp_id), &modifier);
                }
            }
            SsModTgtFilter::LocSrq(dom, srq_id) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_pardom_srq.remove(&(src_fit.id, dom, srq_id), &modifier);
                }
            }
            SsModTgtFilter::OwnSrq(srq) => {
                if let Some(src_fit) = src_fit_opt {
                    self.mods_own_srq.remove(&(src_fit.id, srq), &modifier);
                }
            }
        }
    }
}

fn filter_and_extend<K: Eq + Hash>(
    vec: &mut Vec<SsAttrMod>,
    storage: &KeyedStorage1L<K, SsAttrMod>,
    key: &K,
    attr_id: &EAttrId,
) {
    match storage.get(key) {
        Some(v) => vec.extend(v.iter().filter(|v| &v.tgt_attr_id == attr_id).map(|v| v.clone())),
        _ => (),
    }
}
