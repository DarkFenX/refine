use std::collections::HashSet;

use crate::{
    defs::{EItemGrpId, EItemId, SsFitId, SsItemId},
    shr::ModDomain,
    ss::{
        fit::{SsFit, SsFits},
        item::{SsItem, SsItems},
        svc::svce_calc::modifier::{SsAttrMod, SsModTgtFilter},
    },
    util::{extend_vec_from_storage, KeyedStorage1L},
};

use super::iter_dom_pot::DomsPot;

pub(in crate::ss::svc::svce_calc) struct TargetRegister {
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
}
impl TargetRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            tgts: HashSet::new(),
            tgts_topdom: KeyedStorage1L::new(),
            tgts_pardom: KeyedStorage1L::new(),
            tgts_pardom_grp: KeyedStorage1L::new(),
            tgts_pardom_srq: KeyedStorage1L::new(),
            tgts_own_srq: KeyedStorage1L::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_tgt_items(
        &self,
        modifier: &SsAttrMod,
        tgt_fits: &Vec<&SsFit>,
        items: &SsItems,
    ) -> Vec<SsItemId> {
        let mut tgts = Vec::new();
        let src_item = match items.get_item(&modifier.src_item_id) {
            Ok(i) => i,
            _ => return tgts,
        };
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
                for tgt_fit in tgt_fits.iter() {
                    if check_domain_owner(dom, tgt_fit) {
                        extend_vec_from_storage(&mut tgts, &self.tgts_pardom, &(tgt_fit.id, dom));
                    }
                }
            }
            SsModTgtFilter::LocGrp(dom, grp_id) => {
                for tgt_fit in tgt_fits.iter() {
                    if check_domain_owner(dom, tgt_fit) {
                        extend_vec_from_storage(&mut tgts, &self.tgts_pardom_grp, &(tgt_fit.id, dom, grp_id));
                    }
                }
            }
            SsModTgtFilter::LocSrq(dom, srq_id) => {
                for tgt_fit in tgt_fits.iter() {
                    if check_domain_owner(dom, tgt_fit) {
                        extend_vec_from_storage(&mut tgts, &self.tgts_pardom_srq, &(tgt_fit.id, dom, srq_id));
                    }
                }
            }
            SsModTgtFilter::OwnSrq(srq_id) => {
                for tgt_fit in tgt_fits.iter() {
                    extend_vec_from_storage(&mut tgts, &self.tgts_own_srq, &(tgt_fit.id, srq_id));
                }
            }
        }
        tgts
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_topdom_opt = tgt_item.get_top_domain();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.insert(tgt_item_id);
        if let (Some(tgt_fit), Some(tgt_topdom)) = (tgt_fit_opt, tgt_topdom_opt) {
            self.tgts_topdom.add_entry((tgt_fit.id, tgt_topdom), tgt_item_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom.add_entry((tgt_fit.id, tgt_pardom), tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom_grp
                    .add_entry((tgt_fit.id, tgt_pardom, tgt_grp_id), tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_pardom_srq
                        .add_entry((tgt_fit.id, tgt_pardom, *skill_a_item_id), tgt_item_id);
                }
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_own_srq.add_entry((tgt_fit.id, *skill_a_item_id), tgt_item_id);
                }
            }
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_topdom_opt = tgt_item.get_top_domain();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.insert(tgt_item_id);
        if let (Some(tgt_fit), Some(tgt_topdom)) = (tgt_fit_opt, tgt_topdom_opt) {
            self.tgts_topdom.remove_entry(&(tgt_fit.id, tgt_topdom), &tgt_item_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom.remove_entry(&(tgt_fit.id, tgt_pardom), &tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                self.tgts_pardom_grp
                    .remove_entry(&(tgt_fit.id, tgt_pardom, tgt_grp_id), &tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for tgt_pardom in DomsPot::new(tgt_item) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_pardom_srq
                        .remove_entry(&(tgt_fit.id, tgt_pardom, *skill_a_item_id), &tgt_item_id);
                }
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_own_srq
                        .remove_entry(&(tgt_fit.id, *skill_a_item_id), &tgt_item_id);
                }
            }
        }
    }
}

fn check_domain_owner(dom: ModDomain, fit: &SsFit) -> bool {
    match dom {
        ModDomain::Char => fit.character.is_some(),
        ModDomain::Ship => fit.ship.is_some(),
        ModDomain::Structure => fit.structure.is_some(),
        _ => false,
    }
}
