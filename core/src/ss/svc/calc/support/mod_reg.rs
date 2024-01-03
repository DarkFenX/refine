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

use super::modifier::{SsAttrMod, SsModTgtFilter};

pub(in crate::ss::svc::calc) struct ModRegister {
    // All known target items
    // Contains: HashSet<target item IDs>
    tgts: HashSet<SsItemId>,
    // Items belonging to certain location (e.g. char's implants, ship's modules)
    // Contains: KeyedStorage<target location item ID, target item IDs>
    tgts_loc: KeyedStorage1L<SsItemId, SsItemId>,
    // Items belonging to certain location and group
    // Contains: KeyedStorage<(target location item ID, target's group ID), target item IDs>
    tgts_loc_grp: KeyedStorage1L<(SsItemId, EItemGrpId), SsItemId>,
    // Items belonging to certain location, and having certain skill requirement
    // Contains: KeyedStorage<(target location item ID, target's skillreq type ID), target item IDs>
    tgts_loc_srq: KeyedStorage1L<(SsItemId, EItemId), SsItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), target item IDs>
    tgts_own_srq: KeyedStorage1L<(SsFitId, EItemId), SsItemId>,
    // Modifiers registered for an item
    // Contains: KeyedStorage<modifier item ID, modifiers>
    mods: KeyedStorage1L<SsItemId, SsAttrMod>,
    // Modifiers which modify item directly
    // Contains: KeyedStorage<modified item ID, modifiers>
    mods_direct: KeyedStorage1L<SsItemId, SsAttrMod>,
    // Modifiers influencing all items belonging to certain fit and domain
    // Contains: KeyedStorage<target location item ID, modifiers>
    mods_loc: KeyedStorage1L<SsItemId, SsAttrMod>,
    // Modifiers influencing items belonging to certain fit, domain and group
    // Contains: KeyedStorage<(target location item ID, target's group ID), modifiers>
    mods_loc_grp: KeyedStorage1L<(SsItemId, EItemGrpId), SsAttrMod>,
    // Modifiers influencing items belonging to certain fit and domain, and having certain skill requirement
    // Contains: KeyedStorage<(target location item ID, target's skillreq type ID), modifiers>
    mods_loc_srq: KeyedStorage1L<(SsItemId, EItemId), SsAttrMod>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), modifiers>
    mods_own_srq: KeyedStorage1L<(SsFitId, EItemId), SsAttrMod>,
}
impl ModRegister {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self {
            tgts: HashSet::new(),
            tgts_loc: KeyedStorage1L::new(),
            tgts_loc_grp: KeyedStorage1L::new(),
            tgts_loc_srq: KeyedStorage1L::new(),
            tgts_own_srq: KeyedStorage1L::new(),
            mods: KeyedStorage1L::new(),
            mods_direct: KeyedStorage1L::new(),
            mods_loc: KeyedStorage1L::new(),
            mods_loc_grp: KeyedStorage1L::new(),
            mods_loc_srq: KeyedStorage1L::new(),
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
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => {
                if let Some(dom_item_id) = get_domain_item_id(dom, modifier, src_item, fits) {
                    tgts.push(dom_item_id);
                }
            }
            SsModTgtFilter::Loc(dom) => {
                if let Some(dom_item_id) = get_domain_item_id(dom, modifier, src_item, fits) {
                    extend_vec_from_storage(&mut tgts, &self.tgts_loc, &dom_item_id);
                }
            }
            SsModTgtFilter::LocGrp(dom, grp_id) => {
                if let Some(dom_item_id) = get_domain_item_id(dom, modifier, src_item, fits) {
                    extend_vec_from_storage(&mut tgts, &self.tgts_loc_grp, &(dom_item_id, grp_id));
                }
            }
            SsModTgtFilter::LocSrq(dom, srq_id) => {
                if let Some(dom_item_id) = get_domain_item_id(dom, modifier, src_item, fits) {
                    extend_vec_from_storage(&mut tgts, &self.tgts_loc_srq, &(dom_item_id, srq_id));
                }
            }
            SsModTgtFilter::OwnSrq(srq_id) => {
                if let Some(src_fit_id) = src_item.get_fit_id() {
                    extend_vec_from_storage(&mut tgts, &self.tgts_own_srq, &(src_fit_id, srq_id))
                }
            }
        }
        tgts
    }
    pub(in crate::ss::svc::calc) fn get_mods_for_tgt(&self, tgt_item: &SsItem) -> Vec<SsAttrMod> {
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        let mut mods = Vec::new();
        extend_vec_from_storage(&mut mods, &self.mods_direct, &tgt_item.get_id());
        for domain_item_id_opt in tgt_item.get_parent_domain_item_ids() {
            if let Some(domain_item_id) = domain_item_id_opt {
                extend_vec_from_storage(&mut mods, &self.mods_loc, &domain_item_id);
                if let Ok(tgt_grp_id) = tgt_item.get_group_id() {
                    extend_vec_from_storage(&mut mods, &self.mods_loc_grp, &(domain_item_id, tgt_grp_id));
                };
                if let Ok(tgt_srqs) = tgt_srqs_res {
                    for skill_a_item_id in tgt_srqs.keys() {
                        extend_vec_from_storage(&mut mods, &self.mods_loc_srq, &(domain_item_id, *skill_a_item_id));
                    }
                };
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit_id), Ok(tgt_srqs)) = (tgt_item.get_fit_id(), tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    extend_vec_from_storage(&mut mods, &self.mods_own_srq, &(tgt_fit_id, *skill_a_item_id));
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
    pub(in crate::ss::svc::calc) fn reg_tgt(&mut self, tgt_item: &SsItem) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.insert(tgt_item_id);
        for domain_item_id_opt in tgt_item.get_parent_domain_item_ids() {
            if let Some(domain_item_id) = domain_item_id_opt {
                self.tgts_loc.add(domain_item_id, tgt_item_id);
                if let Ok(tgt_grp_id) = tgt_item.get_group_id() {
                    self.tgts_loc_grp.add((domain_item_id, tgt_grp_id), tgt_item_id);
                };
                if let Ok(tgt_srqs) = tgt_srqs_res {
                    for skill_a_item_id in tgt_srqs.keys() {
                        self.tgts_loc_srq.add((domain_item_id, *skill_a_item_id), tgt_item_id);
                    }
                };
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit_id), Ok(tgt_srqs)) = (tgt_item.get_fit_id(), tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_own_srq.add((tgt_fit_id, *skill_a_item_id), tgt_item_id);
                }
            }
        }
    }
    pub(in crate::ss::svc::calc) fn unreg_tgt(&mut self, tgt_item: &SsItem) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.remove(&tgt_item_id);
        for domain_item_id_opt in tgt_item.get_parent_domain_item_ids() {
            if let Some(domain_item_id) = domain_item_id_opt {
                self.tgts_loc.remove(&domain_item_id, &tgt_item_id);
                if let Ok(tgt_grp_id) = tgt_item.get_group_id() {
                    self.tgts_loc_grp.remove(&(domain_item_id, tgt_grp_id), &tgt_item_id);
                };
                if let Ok(tgt_srqs) = tgt_srqs_res {
                    for skill_a_item_id in tgt_srqs.keys() {
                        self.tgts_loc_srq
                            .remove(&(domain_item_id, *skill_a_item_id), &tgt_item_id);
                    }
                };
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit_id), Ok(tgt_srqs)) = (tgt_item.get_fit_id(), tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_own_srq.remove(&(tgt_fit_id, *skill_a_item_id), &tgt_item_id);
                }
            }
        }
    }
    pub(in crate::ss::svc::calc) fn reg_mods(&mut self, mods: Vec<SsAttrMod>, items: &SsItems, fits: &SsFits) {
        for modifier in mods {
            self.mods.add(modifier.src_item_id, modifier);
            let src_item = match items.get_item(&modifier.src_item_id) {
                Ok(item) => item,
                _ => continue,
            };
            match modifier.tgt_filter {
                SsModTgtFilter::Direct(dom) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_direct.add(dom_item_id, modifier);
                    }
                }
                SsModTgtFilter::Loc(dom) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_loc.add(dom_item_id, modifier);
                    }
                }
                SsModTgtFilter::LocGrp(dom, grp_id) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_loc_grp.add((dom_item_id, grp_id), modifier);
                    }
                }
                SsModTgtFilter::LocSrq(dom, srq_id) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_loc_srq.add((dom_item_id, srq_id), modifier);
                    }
                }
                SsModTgtFilter::OwnSrq(srq_id) => {
                    if let Some(src_fit_id) = src_item.get_fit_id() {
                        self.mods_own_srq.add((src_fit_id, srq_id), modifier);
                    }
                }
            }
        }
    }
    pub(in crate::ss::svc::calc) fn unreg_mods(&mut self, mods: Vec<SsAttrMod>, items: &SsItems, fits: &SsFits) {
        for modifier in mods {
            self.mods.remove(&modifier.src_item_id, &modifier);
            let src_item = match items.get_item(&modifier.src_item_id) {
                Ok(item) => item,
                _ => continue,
            };
            match modifier.tgt_filter {
                SsModTgtFilter::Direct(dom) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_direct.remove(&dom_item_id, &modifier);
                    }
                }
                SsModTgtFilter::Loc(dom) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_loc.remove(&dom_item_id, &modifier);
                    }
                }
                SsModTgtFilter::LocGrp(dom, grp_id) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_loc_grp.remove(&(dom_item_id, grp_id), &modifier);
                    }
                }
                SsModTgtFilter::LocSrq(dom, srq_id) => {
                    if let Some(dom_item_id) = get_domain_item_id(dom, &modifier, src_item, fits) {
                        self.mods_loc_srq.remove(&(dom_item_id, srq_id), &modifier);
                    }
                }
                SsModTgtFilter::OwnSrq(srq_id) => {
                    if let Some(src_fit_id) = src_item.get_fit_id() {
                        self.mods_own_srq.remove(&(src_fit_id, srq_id), &modifier);
                    }
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

// Fetches ID of an item which represents specific domain
fn get_domain_item_id(mod_domain: ModDomain, modifier: &SsAttrMod, item: &SsItem, fits: &SsFits) -> Option<SsItemId> {
    match mod_domain {
        ModDomain::Item => Some(modifier.src_item_id),
        ModDomain::Char => get_item_fit(item, fits).map_or(None, |v| v.character),
        ModDomain::Ship => get_item_fit(item, fits).map_or(None, |v| v.ship),
        ModDomain::Structure => get_item_fit(item, fits).map_or(None, |v| v.structure),
        ModDomain::Other => item.get_other(),
    }
}

fn get_item_fit<'a>(item: &SsItem, fits: &'a SsFits) -> Option<&'a SsFit> {
    item.get_fit_id().map_or(None, |v| fits.get_fit(&v).ok())
}
