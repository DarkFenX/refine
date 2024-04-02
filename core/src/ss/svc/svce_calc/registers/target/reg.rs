use std::{collections::HashSet, convert::TryInto};

use crate::{
    defs::{EItemGrpId, EItemId, SsFitId, SsItemId},
    ss::{
        fit::{SsFit, SsFits},
        item::SsItem,
        svc::svce_calc::{
            modifier::{SsAttrMod, SsModDomain, SsModTgtFilter, SsModType},
            SsLocType,
        },
        SsView,
    },
    util::{extend_vec_from_storage, KeyedStorage1L},
};

use super::iter_loc_pot::LocsPot;

pub(in crate::ss::svc::svce_calc) struct TargetRegister {
    // All known target items
    // Contains: HashSet<target item IDs>
    tgts: HashSet<SsItemId>,
    // Items which are holders of a location type (like char, ship)
    // Contains: KeyedStorage<(target's fit ID, target's location type), target item IDs>
    // TODO: check if we need keyed storage over hashmap here, and check if we need it altogether
    tgts_root: KeyedStorage1L<(SsFitId, SsLocType), SsItemId>,
    // Items belonging to certain fit and location type (e.g. char's implants, ship's modules)
    // Contains: KeyedStorage<(target's fit ID, target's location type), target item IDs>
    tgts_loc: KeyedStorage1L<(SsFitId, SsLocType), SsItemId>,
    // Items belonging to certain fit, location type and group
    // Contains: KeyedStorage<(target's fit ID, target's location type, target's group ID), target item IDs>
    tgts_loc_grp: KeyedStorage1L<(SsFitId, SsLocType, EItemGrpId), SsItemId>,
    // Items belonging to certain fit and location type, and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's location type, target's skillreq type ID), target item IDs>
    tgts_loc_srq: KeyedStorage1L<(SsFitId, SsLocType, EItemId), SsItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), target item IDs>
    tgts_own_srq: KeyedStorage1L<(SsFitId, EItemId), SsItemId>,
    // Everything-buff-modifiable items which belong to certain fit
    // Contains: KeyedStorage<target's fit ID, target item IDs>
    tgts_buff_all: KeyedStorage1L<SsFitId, SsItemId>,
}
impl TargetRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            tgts: HashSet::new(),
            tgts_root: KeyedStorage1L::new(),
            tgts_loc: KeyedStorage1L::new(),
            tgts_loc_grp: KeyedStorage1L::new(),
            tgts_loc_srq: KeyedStorage1L::new(),
            tgts_own_srq: KeyedStorage1L::new(),
            tgts_buff_all: KeyedStorage1L::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_tgt_items(
        &self,
        ss_view: &SsView,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
    ) -> Vec<SsItemId> {
        let mut tgts = Vec::new();
        let src_item = match ss_view.items.get_item(&modifier.src_item_id) {
            Ok(i) => i,
            _ => return tgts,
        };
        let mut tgt_fits = Vec::new();
        match modifier.mod_type {
            SsModType::Local | SsModType::FitWide => {
                if let Some(tgt_fit_id) = mod_item.get_fit_id() {
                    if let Ok(tgt_fit) = ss_view.fits.get_fit(&tgt_fit_id) {
                        tgt_fits.push(tgt_fit);
                    }
                }
            }
            SsModType::SystemWide => tgt_fits.extend(ss_view.fits.iter_fits()),
            SsModType::Projected => (),
            SsModType::Fleet => (),
        }
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_buff_all, &tgt_fit.id)
                    }
                }
                SsModDomain::Item => tgts.push(modifier.src_item_id),
                SsModDomain::Char => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_root, &(tgt_fit.id, SsLocType::Character));
                    }
                }
                SsModDomain::Ship => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_root, &(tgt_fit.id, SsLocType::Ship));
                    }
                }
                SsModDomain::Structure => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_storage(&mut tgts, &self.tgts_root, &(tgt_fit.id, SsLocType::Structure));
                    }
                }
                SsModDomain::Other => {
                    if let Some(other_item_id) = src_item.get_other() {
                        tgts.push(other_item_id);
                    }
                }
            },
            SsModTgtFilter::Loc(dom) => {
                if let Ok(loc) = dom.try_into() {
                    for tgt_fit in tgt_fits.iter() {
                        if check_domain_owner(dom, tgt_fit) {
                            extend_vec_from_storage(&mut tgts, &self.tgts_loc, &(tgt_fit.id, loc));
                        }
                    }
                }
            }
            SsModTgtFilter::LocGrp(dom, grp_id) => {
                if let Ok(loc) = dom.try_into() {
                    for tgt_fit in tgt_fits.iter() {
                        if check_domain_owner(dom, tgt_fit) {
                            extend_vec_from_storage(&mut tgts, &self.tgts_loc_grp, &(tgt_fit.id, loc, grp_id));
                        }
                    }
                }
            }
            SsModTgtFilter::LocSrq(dom, srq_id) => {
                if let Ok(loc) = dom.try_into() {
                    for tgt_fit in tgt_fits.iter() {
                        if check_domain_owner(dom, tgt_fit) {
                            extend_vec_from_storage(&mut tgts, &self.tgts_loc_srq, &(tgt_fit.id, loc, srq_id));
                        }
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
        let tgt_root_loc_opt = tgt_item.get_root_loc_type();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.insert(tgt_item_id);
        if let (Some(tgt_fit), Some(tgt_root_loc)) = (tgt_fit_opt, tgt_root_loc_opt) {
            self.tgts_root.add_entry((tgt_fit.id, tgt_root_loc), tgt_item_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for tgt_loc in LocsPot::new(tgt_item) {
                self.tgts_loc.add_entry((tgt_fit.id, tgt_loc), tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for tgt_pardom in LocsPot::new(tgt_item) {
                self.tgts_loc_grp
                    .add_entry((tgt_fit.id, tgt_pardom, tgt_grp_id), tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for tgt_pardom in LocsPot::new(tgt_item) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_loc_srq
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
        if tgt_item.is_buff_modifiable() {
            if let Some(tgt_fit) = tgt_fit_opt {
                self.tgts_buff_all.add_entry(tgt_fit.id, tgt_item_id);
            }
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_topdom_opt = tgt_item.get_root_loc_type();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        self.tgts.insert(tgt_item_id);
        if let (Some(tgt_fit), Some(tgt_topdom)) = (tgt_fit_opt, tgt_topdom_opt) {
            self.tgts_root.remove_entry(&(tgt_fit.id, tgt_topdom), &tgt_item_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for tgt_pardom in LocsPot::new(tgt_item) {
                self.tgts_loc.remove_entry(&(tgt_fit.id, tgt_pardom), &tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for tgt_pardom in LocsPot::new(tgt_item) {
                self.tgts_loc_grp
                    .remove_entry(&(tgt_fit.id, tgt_pardom, tgt_grp_id), &tgt_item_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for tgt_pardom in LocsPot::new(tgt_item) {
                for skill_a_item_id in tgt_srqs.keys() {
                    self.tgts_loc_srq
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
        if tgt_item.is_buff_modifiable() {
            if let Some(tgt_fit) = tgt_fit_opt {
                self.tgts_buff_all.remove_entry(&tgt_fit.id, &tgt_item_id);
            }
        }
    }
}

fn check_domain_owner(dom: SsModDomain, fit: &SsFit) -> bool {
    match dom {
        SsModDomain::Char => fit.character.is_some(),
        SsModDomain::Ship => fit.ship.is_some(),
        SsModDomain::Structure => fit.structure.is_some(),
        _ => false,
    }
}
