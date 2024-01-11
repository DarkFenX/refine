use std::{collections::HashSet, hash::Hash};

use crate::{
    defs::{EItemGrpId, EItemId, SsFitId, SsItemId},
    shr::ModDomain,
    ss::{
        fit::{SsFit, SsFits},
        item::{SsItem, SsItems},
    },
    util::KeyedStorage1L,
};

use super::{
    super::modifier::{SsAttrMod, SsModTgtFilter},
    DomsAct, DomsPot,
};

pub(in crate::ss::svc::calc) struct ModRegister {
    // All known target items
    // Contains: HashSet<target item IDs>
    tgts: HashSet<SsItemId>,
    // Top-level items which are representing an "owner" of domain (char, ship)
    // Contains: KeyedStorage<(target's fit ID, target's domain), target item IDs>
    tgts_topdom: KeyedStorage1L<(SsFitId, ModDomain), SsItemId>,
    // Items belonging to certain fit and domain (e.g. char's implants, ship's modules)
    // Contains: KeyedStorage<(target's fit ID, target's domain), target item IDs>
    tgts_pardom: KeyedStorage1L<(SsFitId, ModDomain), SsItemId>,
    // Items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(target's fit ID, target's domain, target's group ID), target item IDs>
    tgts_pardom_grp: KeyedStorage1L<(SsFitId, ModDomain, EItemGrpId), SsItemId>,
    // Items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's domain, target's skillreq type ID), target item IDs>
    tgts_pardom_srq: KeyedStorage1L<(SsFitId, ModDomain, EItemId), SsItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), target item IDs>
    tgts_own_srq: KeyedStorage1L<(SsFitId, EItemId), SsItemId>,
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
impl ModRegister {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self {
            tgts: HashSet::new(),
            tgts_topdom: KeyedStorage1L::new(),
            tgts_pardom: KeyedStorage1L::new(),
            tgts_pardom_grp: KeyedStorage1L::new(),
            tgts_pardom_srq: KeyedStorage1L::new(),
            tgts_own_srq: KeyedStorage1L::new(),
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
    pub(in crate::ss::svc::calc) fn get_tgt_items(
        &self,
        modifier: &SsAttrMod,
        items: &SsItems,
        fits: &SsFits,
    ) -> Vec<SsItemId> {
        let mut tgts = Vec::new();
        let src_item = match items.get_item(&modifier.src_item_id) {
            Ok(i) => i,
            _ => return tgts,
        };
        let src_fit_opt = src_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                ModDomain::Item => tgts.push(modifier.src_item_id),
                ModDomain::Char | ModDomain::Ship | ModDomain::Structure => {
                    if let Some(src_fit_id) = src_item.get_fit_id() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_topdom, &(src_fit_id, dom));
                    }
                }
                ModDomain::Other => {
                    if let Some(other_item_id) = src_item.get_other() {
                        tgts.push(other_item_id);
                    }
                }
            },
            SsModTgtFilter::Loc(dom) => {
                if check_domain_owner(dom, src_fit_opt) {
                    if let Some(src_fit_id) = src_item.get_fit_id() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_pardom, &(src_fit_id, dom));
                    }
                }
            }
            SsModTgtFilter::LocGrp(dom, grp_id) => {
                if check_domain_owner(dom, src_fit_opt) {
                    if let Some(src_fit_id) = src_item.get_fit_id() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_pardom_grp, &(src_fit_id, dom, grp_id));
                    }
                }
            }
            SsModTgtFilter::LocSrq(dom, srq_id) => {
                if check_domain_owner(dom, src_fit_opt) {
                    if let Some(src_fit_id) = src_item.get_fit_id() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_pardom_srq, &(src_fit_id, dom, srq_id));
                    }
                }
            }
            SsModTgtFilter::OwnSrq(srq_id) => {
                if let Some(src_fit_id) = src_item.get_fit_id() {
                    extend_vec_from_storage(&mut tgts, &self.tgts_own_srq, &(src_fit_id, srq_id));
                }
            }
        }
        tgts
    }
    pub(in crate::ss::svc::calc) fn get_mods_for_tgt(&self, tgt_item: &SsItem, fits: &SsFits) -> Vec<SsAttrMod> {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_topdom_opt = tgt_item.get_top_domain();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        let mut mods = Vec::new();
        extend_vec_from_storage(&mut mods, &self.mods_direct, &tgt_item_id);
        if let (Some(tgt_fit), Some(tgt_topdom)) = (tgt_fit_opt, tgt_topdom_opt) {
            extend_vec_from_storage(&mut mods, &self.mods_topdom, &(tgt_fit.id, tgt_topdom));
        }
        if let Some(other_item_id) = tgt_item.get_other() {
            extend_vec_from_storage(&mut mods, &self.mods_other, &other_item_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for dom in DomsAct::new(tgt_item, tgt_fit) {
                extend_vec_from_storage(&mut mods, &self.mods_pardom, &(tgt_fit.id, dom));
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for dom in DomsAct::new(tgt_item, tgt_fit) {
                extend_vec_from_storage(&mut mods, &self.mods_pardom_grp, &(tgt_fit.id, dom, tgt_grp_id));
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for dom in DomsAct::new(tgt_item, tgt_fit) {
                for skill_a_item_id in tgt_srqs.keys() {
                    extend_vec_from_storage(&mut mods, &self.mods_pardom_srq, &(tgt_fit.id, dom, *skill_a_item_id));
                }
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    extend_vec_from_storage(&mut mods, &self.mods_own_srq, &(tgt_fit.id, *skill_a_item_id));
                }
            }
        }
        mods
    }
    pub(in crate::ss::svc::calc) fn get_mods_for_changed_domain_owner(
        &mut self,
        item: &SsItem,
        items: &SsItems,
    ) -> Vec<SsAttrMod> {
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
    pub(in crate::ss::svc::calc) fn iter_mods_for_src(
        &self,
        src_item_id: &SsItemId,
    ) -> impl Iterator<Item = &SsAttrMod> {
        self.mods.get(src_item_id).into_iter().flatten()
    }
    // Modification methods
    pub(in crate::ss::svc::calc) fn reg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_topdom_opt = tgt_item.get_top_domain();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.insert(tgt_item_id);
        if let (Some(tgt_fit), Some(tgt_topdom)) = (tgt_fit_opt, tgt_topdom_opt) {
            self.tgts_topdom.add((tgt_fit.id, tgt_topdom), tgt_item_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom.add((tgt_fit.id, tgt_pardom), tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom_grp
                    .add((tgt_fit.id, tgt_pardom, tgt_grp_id), tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_pardom_srq
                        .add((tgt_fit.id, tgt_pardom, *skill_a_item_id), tgt_item_id);
                }
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_own_srq.add((tgt_fit.id, *skill_a_item_id), tgt_item_id);
                }
            }
        }
    }
    pub(in crate::ss::svc::calc) fn unreg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_topdom_opt = tgt_item.get_top_domain();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.insert(tgt_item_id);
        if let (Some(tgt_fit), Some(tgt_topdom)) = (tgt_fit_opt, tgt_topdom_opt) {
            self.tgts_topdom.remove(&(tgt_fit.id, tgt_topdom), &tgt_item_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom.remove(&(tgt_fit.id, tgt_pardom), &tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom_grp
                    .remove(&(tgt_fit.id, tgt_pardom, tgt_grp_id), &tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_pardom_srq
                        .remove(&(tgt_fit.id, tgt_pardom, *skill_a_item_id), &tgt_item_id);
                }
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_own_srq.remove(&(tgt_fit.id, *skill_a_item_id), &tgt_item_id);
                }
            }
        }
    }
    pub(in crate::ss::svc::calc) fn reg_mod(&mut self, src_fit_opt: Option<&SsFit>, modifier: SsAttrMod) {
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
    pub(in crate::ss::svc::calc) fn unreg_mod(&mut self, src_fit_opt: Option<&SsFit>, modifier: &SsAttrMod) {
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

fn extend_vec_from_storage<K: Eq + Hash, V: Eq + Hash + Clone>(
    vec: &mut Vec<V>,
    storage: &KeyedStorage1L<K, V>,
    key: &K,
) {
    match storage.get(key) {
        Some(v) => vec.extend(v.iter().map(|v| v.clone())),
        _ => (),
    }
}

fn check_domain_owner(dom: ModDomain, fit_opt: Option<&SsFit>) -> bool {
    match fit_opt {
        None => false,
        Some(fit) => match dom {
            ModDomain::Char => fit.character.is_some(),
            ModDomain::Ship => fit.ship.is_some(),
            ModDomain::Structure => fit.structure.is_some(),
            _ => false,
        },
    }
}
