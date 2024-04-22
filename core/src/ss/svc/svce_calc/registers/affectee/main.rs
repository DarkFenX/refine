use std::convert::TryInto;

use crate::{
    defs::{EItemGrpId, EItemId, SsFitId, SsItemId},
    ss::{
        fit::SsFit,
        item::SsItem,
        svc::svce_calc::{SsAttrMod, SsLocType, SsModDomain, SsModTgtFilter, SsModType},
        SsView,
    },
    util::{extend_vec_from_map_set_l1, StMapSetL1},
};

use super::PotentialLocations;

pub(in crate::ss::svc::svce_calc) struct SolAffecteeRegister {
    // Items which are holders of a location type (like char, ship)
    // Map<(affectee fit ID, affectee location type), affectee item IDs>
    pub(super) root: StMapSetL1<(SsFitId, SsLocType), SsItemId>,
    // Items belonging to certain fit and location type (e.g. char's implants, ship's modules)
    // Map<(affectee fit ID, affectee location type), affectee item IDs>
    pub(super) loc: StMapSetL1<(SsFitId, SsLocType), SsItemId>,
    // Items belonging to certain fit, location type and group
    // Map<(affectee fit ID, affectee location type, affectee group ID), affectee item IDs>
    pub(super) loc_grp: StMapSetL1<(SsFitId, SsLocType, EItemGrpId), SsItemId>,
    // Items belonging to certain fit and location type, and having certain skill requirement
    // Map<(affectee fit ID, affectee location type, affectee skillreq type ID), affectee item IDs>
    pub(super) loc_srq: StMapSetL1<(SsFitId, SsLocType, EItemId), SsItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Map<(affectee fit ID, affectee skillreq type ID), affectee item IDs>
    pub(super) own_srq: StMapSetL1<(SsFitId, EItemId), SsItemId>,
    // Everything-buff-modifiable items which belong to certain fit
    // Map<affectee fit ID, affectee item IDs>
    pub(super) buff_all: StMapSetL1<SsFitId, SsItemId>,
}
impl SolAffecteeRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            root: StMapSetL1::new(),
            loc: StMapSetL1::new(),
            loc_grp: StMapSetL1::new(),
            loc_srq: StMapSetL1::new(),
            own_srq: StMapSetL1::new(),
            buff_all: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_affectees(
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
            SsModType::Targeted => (),
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
        self.get_affectees_for_fits(mod_item, modifier, &tgt_fits)
    }
    pub(in crate::ss::svc::svce_calc) fn get_affectees_for_fits(
        &self,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
        tgt_fits: &Vec<&SsFit>,
    ) -> Vec<SsItemId> {
        let mut affectees = Vec::new();
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut affectees, &self.buff_all, &tgt_fit.id)
                    }
                }
                SsModDomain::Item => affectees.push(modifier.src_item_id),
                SsModDomain::Char => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut affectees, &self.root, &(tgt_fit.id, SsLocType::Character));
                    }
                }
                SsModDomain::Ship => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut affectees, &self.root, &(tgt_fit.id, SsLocType::Ship));
                    }
                }
                SsModDomain::Structure => {
                    for tgt_fit in tgt_fits.iter() {
                        extend_vec_from_map_set_l1(&mut affectees, &self.root, &(tgt_fit.id, SsLocType::Structure));
                    }
                }
                SsModDomain::Other => {
                    if let Some(other_item_id) = mod_item.get_other() {
                        affectees.push(other_item_id);
                    }
                }
                SsModDomain::Target => (),
            },
            SsModTgtFilter::Loc(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit in tgt_fits.iter() {
                        if check_domain_owner(SsModDomain::Ship, tgt_fit) {
                            extend_vec_from_map_set_l1(&mut affectees, &self.loc, &(tgt_fit.id, SsLocType::Ship));
                        }
                        if check_domain_owner(SsModDomain::Structure, tgt_fit) {
                            extend_vec_from_map_set_l1(&mut affectees, &self.loc, &(tgt_fit.id, SsLocType::Structure));
                        }
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit in tgt_fits.iter() {
                            if check_domain_owner(dom, tgt_fit) {
                                extend_vec_from_map_set_l1(&mut affectees, &self.loc, &(tgt_fit.id, loc));
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
                                &mut affectees,
                                &self.loc_grp,
                                &(tgt_fit.id, SsLocType::Ship, grp_id),
                            );
                        }
                        if check_domain_owner(SsModDomain::Structure, tgt_fit) {
                            extend_vec_from_map_set_l1(
                                &mut affectees,
                                &self.loc_grp,
                                &(tgt_fit.id, SsLocType::Structure, grp_id),
                            );
                        }
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit in tgt_fits.iter() {
                            if check_domain_owner(dom, tgt_fit) {
                                extend_vec_from_map_set_l1(&mut affectees, &self.loc_grp, &(tgt_fit.id, loc, grp_id));
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
                                &mut affectees,
                                &self.loc_srq,
                                &(tgt_fit.id, SsLocType::Ship, srq_id),
                            );
                        }
                        if check_domain_owner(SsModDomain::Structure, tgt_fit) {
                            extend_vec_from_map_set_l1(
                                &mut affectees,
                                &self.loc_srq,
                                &(tgt_fit.id, SsLocType::Structure, srq_id),
                            );
                        }
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit in tgt_fits.iter() {
                            if check_domain_owner(dom, tgt_fit) {
                                extend_vec_from_map_set_l1(&mut affectees, &self.loc_srq, &(tgt_fit.id, loc, srq_id));
                            }
                        }
                    }
                }
            },
            SsModTgtFilter::OwnSrq(srq_id) => {
                for tgt_fit in tgt_fits.iter() {
                    extend_vec_from_map_set_l1(&mut affectees, &self.own_srq, &(tgt_fit.id, srq_id));
                }
            }
        }
        affectees
    }
    pub(in crate::ss::svc::svce_calc) fn get_affectees_for_tgt_item(
        &self,
        modifier: &SsAttrMod,
        tgt_item: &SsItem,
    ) -> Vec<SsItemId> {
        let mut affectees = Vec::new();
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything | SsModDomain::Target => affectees.push(tgt_item.get_id()),
                SsModDomain::Ship if matches!(tgt_item, SsItem::Ship(_)) => affectees.push(tgt_item.get_id()),
                SsModDomain::Structure if matches!(tgt_item, SsItem::Structure(_)) => affectees.push(tgt_item.get_id()),
                _ => (),
            },
            SsModTgtFilter::Loc(dom) => match dom {
                SsModDomain::Everything | SsModDomain::Target => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(&mut affectees, &self.loc, &(ship.fit_id, SsLocType::Ship))
                    }
                    SsItem::Structure(structure) => {
                        extend_vec_from_map_set_l1(&mut affectees, &self.loc, &(structure.fit_id, SsLocType::Structure))
                    }
                    _ => (),
                },
                SsModDomain::Ship => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(&mut affectees, &self.loc, &(ship.fit_id, SsLocType::Ship))
                    }
                    _ => (),
                },
                SsModDomain::Structure => match tgt_item {
                    SsItem::Structure(structure) => {
                        extend_vec_from_map_set_l1(&mut affectees, &self.loc, &(structure.fit_id, SsLocType::Structure))
                    }
                    _ => (),
                },
                _ => (),
            },
            SsModTgtFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Everything | SsModDomain::Target => match tgt_item {
                    SsItem::Ship(ship) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_grp,
                        &(ship.fit_id, SsLocType::Ship, grp_id),
                    ),
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_grp,
                        &(structure.fit_id, SsLocType::Structure, grp_id),
                    ),
                    _ => (),
                },
                SsModDomain::Ship => match tgt_item {
                    SsItem::Ship(ship) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_grp,
                        &(ship.fit_id, SsLocType::Ship, grp_id),
                    ),
                    _ => (),
                },
                SsModDomain::Structure => match tgt_item {
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_grp,
                        &(structure.fit_id, SsLocType::Structure, grp_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            SsModTgtFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Everything | SsModDomain::Target => match tgt_item {
                    SsItem::Ship(ship) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_srq,
                        &(ship.fit_id, SsLocType::Ship, srq_id),
                    ),
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_srq,
                        &(structure.fit_id, SsLocType::Structure, srq_id),
                    ),
                    _ => (),
                },
                SsModDomain::Ship => match tgt_item {
                    SsItem::Ship(ship) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_srq,
                        &(ship.fit_id, SsLocType::Ship, srq_id),
                    ),
                    _ => (),
                },
                SsModDomain::Structure => match tgt_item {
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        &mut affectees,
                        &self.loc_srq,
                        &(structure.fit_id, SsLocType::Structure, srq_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            SsModTgtFilter::OwnSrq(srq_id) => match tgt_item {
                SsItem::Ship(ship) => extend_vec_from_map_set_l1(&mut affectees, &self.own_srq, &(ship.fit_id, srq_id)),
                SsItem::Structure(structure) => {
                    extend_vec_from_map_set_l1(&mut affectees, &self.own_srq, &(structure.fit_id, srq_id))
                }
                _ => (),
            },
        }
        affectees
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_affectee(&mut self, ss_view: &SsView, item: &SsItem) {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| ss_view.fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_type();
        let grp_id_opt = item.get_group_id().ok();
        let srqs_opt = item.get_skill_reqs().ok();
        if let (Some(fit), Some(loc_type)) = (fit_opt, root_loc_opt) {
            self.root.add_entry((fit.id, loc_type), item_id);
        }
        if let Some(fit) = fit_opt {
            for loc_type in PotentialLocations::new(item) {
                self.loc.add_entry((fit.id, loc_type), item_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc_type in PotentialLocations::new(item) {
                self.loc_grp.add_entry((fit.id, loc_type, grp_id), item_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc_type in PotentialLocations::new(item) {
                for srq_id in srqs.keys() {
                    self.loc_srq.add_entry((fit.id, loc_type, *srq_id), item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for skill_a_item_id in srqs.keys() {
                    self.own_srq.add_entry((fit.id, *skill_a_item_id), item_id);
                }
            }
        }
        if item.is_buff_modifiable() {
            if let Some(fit) = fit_opt {
                self.buff_all.add_entry(fit.id, item_id);
            }
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_affectee(&mut self, ss_view: &SsView, item: &SsItem) {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| ss_view.fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_type();
        let grp_id_opt = item.get_group_id().ok();
        let srqs_opt = item.get_skill_reqs().ok();
        if let (Some(fit), Some(loc_type)) = (fit_opt, root_loc_opt) {
            self.root.remove_entry(&(fit.id, loc_type), &item_id);
        }
        if let Some(fit) = fit_opt {
            for loc_type in PotentialLocations::new(item) {
                self.loc.remove_entry(&(fit.id, loc_type), &item_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc_type in PotentialLocations::new(item) {
                self.loc_grp.remove_entry(&(fit.id, loc_type, grp_id), &item_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc_type in PotentialLocations::new(item) {
                for srq_id in srqs.keys() {
                    self.loc_srq.remove_entry(&(fit.id, loc_type, *srq_id), &item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for srq_id in srqs.keys() {
                    self.own_srq.remove_entry(&(fit.id, *srq_id), &item_id);
                }
            }
        }
        if item.is_buff_modifiable() {
            if let Some(fit) = fit_opt {
                self.buff_all.remove_entry(&fit.id, &item_id);
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
