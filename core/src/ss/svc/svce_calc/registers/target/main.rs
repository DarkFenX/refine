use std::convert::TryInto;

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
    util::{extend_vec_from_map_set_l1, StMapSetL1},
};

use super::iter_loc_pot::LocsPot;

pub(in crate::ss::svc::svce_calc) struct TargetRegister {
    // Items which are holders of a location type (like char, ship)
    // Contains: KeyedStorage<(target's fit ID, target's location type), target item IDs>
    pub(super) tgts_root: StMapSetL1<(SsFitId, SsLocType), SsItemId>,
    // Items belonging to certain fit and location type (e.g. char's implants, ship's modules)
    // Contains: KeyedStorage<(target's fit ID, target's location type), target item IDs>
    pub(super) tgts_loc: StMapSetL1<(SsFitId, SsLocType), SsItemId>,
    // Items belonging to certain fit, location type and group
    // Contains: KeyedStorage<(target's fit ID, target's location type, target's group ID), target item IDs>
    pub(super) tgts_loc_grp: StMapSetL1<(SsFitId, SsLocType, EItemGrpId), SsItemId>,
    // Items belonging to certain fit and location type, and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's location type, target's skillreq type ID), target item IDs>
    pub(super) tgts_loc_srq: StMapSetL1<(SsFitId, SsLocType, EItemId), SsItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), target item IDs>
    pub(super) tgts_own_srq: StMapSetL1<(SsFitId, EItemId), SsItemId>,
    // Everything-buff-modifiable items which belong to certain fit
    // Contains: KeyedStorage<target's fit ID, target item IDs>
    pub(super) tgts_buff_all: StMapSetL1<SsFitId, SsItemId>,
}
impl TargetRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            tgts_root: StMapSetL1::new(),
            tgts_loc: StMapSetL1::new(),
            tgts_loc_grp: StMapSetL1::new(),
            tgts_loc_srq: StMapSetL1::new(),
            tgts_own_srq: StMapSetL1::new(),
            tgts_buff_all: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_tgt_items_for_fits(
        &self,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
        tgt_fits: &Vec<&SsFit>,
    ) -> Vec<SsItemId> {
        let mut tgts = Vec::new();
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut tgts, &self.tgts_buff_all, &tgt_fit.id)
                    }
                }
                SsModDomain::Item => tgts.push(modifier.src_item_id),
                SsModDomain::Char => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut tgts, &self.tgts_root, &(tgt_fit.id, SsLocType::Character));
                    }
                }
                SsModDomain::Ship => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut tgts, &self.tgts_root, &(tgt_fit.id, SsLocType::Ship));
                    }
                }
                SsModDomain::Structure => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut tgts, &self.tgts_root, &(tgt_fit.id, SsLocType::Structure));
                    }
                }
                SsModDomain::Other => {
                    if let Some(other_item_id) = mod_item.get_other() {
                        tgts.push(other_item_id);
                    }
                }
            },
            SsModTgtFilter::Loc(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit in tgt_fits.iter() {
                        if check_domain_owner(SsModDomain::Ship, tgt_fit) {
                            extend_vec_from_map_set_l1(&mut tgts, &self.tgts_loc, &(tgt_fit.id, SsLocType::Ship));
                        }
                        if check_domain_owner(SsModDomain::Structure, tgt_fit) {
                            extend_vec_from_map_set_l1(&mut tgts, &self.tgts_loc, &(tgt_fit.id, SsLocType::Structure));
                        }
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit in tgt_fits.iter() {
                            if check_domain_owner(dom, tgt_fit) {
                                extend_vec_from_map_set_l1(&mut tgts, &self.tgts_loc, &(tgt_fit.id, loc));
                            }
                        }
                    }
                }
            },
            SsModTgtFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit in tgt_fits.iter() {
                        if check_domain_owner(SsModDomain::Ship, tgt_fit) {
                            extend_vec_from_map_set_l1(
                                &mut tgts,
                                &self.tgts_loc_grp,
                                &(tgt_fit.id, SsLocType::Ship, grp_id),
                            );
                        }
                        if check_domain_owner(SsModDomain::Structure, tgt_fit) {
                            extend_vec_from_map_set_l1(
                                &mut tgts,
                                &self.tgts_loc_grp,
                                &(tgt_fit.id, SsLocType::Structure, grp_id),
                            );
                        }
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit in tgt_fits.iter() {
                            if check_domain_owner(dom, tgt_fit) {
                                extend_vec_from_map_set_l1(&mut tgts, &self.tgts_loc_grp, &(tgt_fit.id, loc, grp_id));
                            }
                        }
                    }
                }
            },
            SsModTgtFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit in tgt_fits.iter() {
                        if check_domain_owner(SsModDomain::Ship, tgt_fit) {
                            extend_vec_from_map_set_l1(
                                &mut tgts,
                                &self.tgts_loc_srq,
                                &(tgt_fit.id, SsLocType::Ship, srq_id),
                            );
                        }
                        if check_domain_owner(SsModDomain::Structure, tgt_fit) {
                            extend_vec_from_map_set_l1(
                                &mut tgts,
                                &self.tgts_loc_srq,
                                &(tgt_fit.id, SsLocType::Structure, srq_id),
                            );
                        }
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit in tgt_fits.iter() {
                            if check_domain_owner(dom, tgt_fit) {
                                extend_vec_from_map_set_l1(&mut tgts, &self.tgts_loc_srq, &(tgt_fit.id, loc, srq_id));
                            }
                        }
                    }
                }
            },
            SsModTgtFilter::OwnSrq(srq_id) => {
                for tgt_fit in tgt_fits.iter() {
                    extend_vec_from_map_set_l1(&mut tgts, &self.tgts_own_srq, &(tgt_fit.id, srq_id));
                }
            }
        }
        tgts
    }
    pub(in crate::ss::svc::svce_calc) fn get_tgt_items(
        &self,
        ss_view: &SsView,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
    ) -> Vec<SsItemId> {
        let mut tgt_fits = Vec::new();
        match modifier.mod_type {
            SsModType::Local | SsModType::FitWide => {
                if let Some(src_fit_id) = mod_item.get_fit_id() {
                    let src_fit = ss_view.fits.get_fit(&src_fit_id).unwrap();
                    tgt_fits.push(src_fit);
                }
            }
            SsModType::SystemWide => tgt_fits.extend(ss_view.fits.iter_fits()),
            SsModType::Projected => (),
            SsModType::Fleet => {
                if let Some(src_fit_id) = mod_item.get_fit_id() {
                    let src_fit = ss_view.fits.get_fit(&src_fit_id).unwrap();
                    match src_fit.fleet {
                        Some(fleet_id) => {
                            let fleet = ss_view.fleets.get_fleet(&fleet_id).unwrap();
                            tgt_fits.extend(fleet.iter_fits().map(|v| ss_view.fits.get_fit(v).unwrap()));
                        }
                        None => tgt_fits.push(src_fit),
                    }
                }
            }
        };
        self.get_tgt_items_for_fits(mod_item, modifier, &tgt_fits)
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_root_loc_opt = tgt_item.get_root_loc_type();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
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
