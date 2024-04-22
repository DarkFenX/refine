use std::convert::TryInto;

use crate::{
    defs::{EItemGrpId, EItemId, SsFitId, SsItemId},
    ss::{
        fit::SsFit,
        item::SsItem,
        svc::svce_calc::{SsAffecteeFilter, SsAttrMod, SsLocType, SsModDomain, SsModType},
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
    pub(in crate::ss::svc::svce_calc) fn fill_affectees(
        &self,
        affectees: &mut Vec<SsItemId>,
        ss_view: &SsView,
        item: &SsItem,
        modifier: &SsAttrMod,
    ) {
        // Those modifiers work the same regardless of broader context. They just need an item which
        // carries them.
        match modifier.affectee_filter {
            SsAffecteeFilter::Direct(dom) => match dom {
                SsModDomain::Item => affectees.push(modifier.affector_item_id),
                SsModDomain::Other => {
                    if let Some(other_item_id) = item.get_other() {
                        affectees.push(other_item_id);
                    }
                }
                _ => (),
            },
            _ => (),
        }
        match modifier.mod_type {
            // Local and fit-wide modifications affect only source fit itself
            SsModType::Local | SsModType::FitWide => {
                let fit = item.get_fit_id().map(|v| ss_view.fits.get_fit(&v).unwrap()).unwrap();
                self.fill_affectees_for_fit(affectees, modifier, fit);
            }
            // System-wide modifications affect all fits
            SsModType::SystemWide => {
                for fit in ss_view.fits.iter_fits() {
                    self.fill_affectees_for_fit(affectees, modifier, fit);
                }
            }
            // Projected and targeted modifications are processed depending on what they target
            SsModType::Projected | SsModType::Targeted => {
                if let Some(tgt_item_ids) = item.iter_targets() {
                    for tgt_item_id in tgt_item_ids {
                        let tgt_item = ss_view.items.get_item(tgt_item_id).unwrap();
                        self.fill_affectees_for_tgt_item(affectees, ss_view, modifier, tgt_item);
                    }
                }
            }
            // Fleet modifications affect whole fleet, or just source fit itself, if fleet isn't set
            SsModType::Fleet => {
                if let Some(src_fit_id) = item.get_fit_id() {
                    let src_fit = ss_view.fits.get_fit(&src_fit_id).unwrap();
                    match src_fit.fleet {
                        Some(fleet_id) => {
                            let fleet = ss_view.fleets.get_fleet(&fleet_id).unwrap();
                            for dst_fit in fleet.iter_fits().map(|v| ss_view.fits.get_fit(v).unwrap()) {
                                self.fill_affectees_for_fit(affectees, modifier, dst_fit);
                            }
                        }
                        None => self.fill_affectees_for_fit(affectees, modifier, src_fit),
                    }
                }
            }
        };
    }
    pub(in crate::ss::svc::svce_calc) fn fill_affectees_for_fit(
        &self,
        affectees: &mut Vec<SsItemId>,
        modifier: &SsAttrMod,
        fit: &SsFit,
    ) {
        match modifier.affectee_filter {
            SsAffecteeFilter::Direct(dom) => match dom {
                SsModDomain::Everything => extend_vec_from_map_set_l1(affectees, &self.buff_all, &fit.id),
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.root, &(fit.id, loc));
                        }
                    }
                }
            },
            SsAffecteeFilter::Loc(dom) => match dom {
                SsModDomain::Everything => {
                    if check_domain_owner(SsModDomain::Ship, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(fit.id, SsLocType::Ship));
                    }
                    if check_domain_owner(SsModDomain::Structure, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(fit.id, SsLocType::Structure));
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.loc, &(fit.id, loc));
                        }
                    }
                }
            },
            SsAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Everything => {
                    if check_domain_owner(SsModDomain::Ship, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(fit.id, SsLocType::Ship, grp_id));
                    }
                    if check_domain_owner(SsModDomain::Structure, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(fit.id, SsLocType::Structure, grp_id));
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(fit.id, loc, grp_id));
                        }
                    }
                }
            },
            SsAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Everything => {
                    if check_domain_owner(SsModDomain::Ship, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(fit.id, SsLocType::Ship, srq_id));
                    }
                    if check_domain_owner(SsModDomain::Structure, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(fit.id, SsLocType::Structure, srq_id));
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(fit.id, loc, srq_id));
                        }
                    }
                }
            },
            SsAffecteeFilter::OwnSrq(srq_id) => {
                extend_vec_from_map_set_l1(affectees, &self.own_srq, &(fit.id, srq_id));
            }
        }
    }
    pub(in crate::ss::svc::svce_calc) fn fill_affectees_for_tgt_item(
        &self,
        affectees: &mut Vec<SsItemId>,
        ss_view: &SsView,
        modifier: &SsAttrMod,
        tgt_item: &SsItem,
    ) {
        match modifier.affectee_filter {
            SsAffecteeFilter::Direct(dom) => match dom {
                SsModDomain::Everything | SsModDomain::Target => affectees.push(tgt_item.get_id()),
                SsModDomain::Ship if matches!(tgt_item, SsItem::Ship(_)) => affectees.push(tgt_item.get_id()),
                SsModDomain::Structure if matches!(tgt_item, SsItem::Structure(_)) => affectees.push(tgt_item.get_id()),
                SsModDomain::Char => match tgt_item {
                    SsItem::Ship(ship) => {
                        if let Some(char_id) = get_fit_character(ss_view, &ship.fit_id) {
                            affectees.push(char_id);
                        }
                    }
                    SsItem::Structure(structure) => {
                        if let Some(char_id) = get_fit_character(ss_view, &structure.fit_id) {
                            affectees.push(char_id);
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            SsAffecteeFilter::Loc(dom) => match dom {
                SsModDomain::Everything | SsModDomain::Target => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(ship.fit_id, SsLocType::Ship))
                    }
                    SsItem::Structure(structure) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(structure.fit_id, SsLocType::Structure))
                    }
                    _ => (),
                },
                SsModDomain::Ship => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(ship.fit_id, SsLocType::Ship))
                    }
                    _ => (),
                },
                SsModDomain::Structure => match tgt_item {
                    SsItem::Structure(structure) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(structure.fit_id, SsLocType::Structure))
                    }
                    _ => (),
                },
                SsModDomain::Char => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(ship.fit_id, SsLocType::Character))
                    }
                    SsItem::Structure(structure) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(structure.fit_id, SsLocType::Character))
                    }
                    _ => (),
                },
                _ => (),
            },
            SsAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Everything | SsModDomain::Target => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(ship.fit_id, SsLocType::Ship, grp_id))
                    }
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(structure.fit_id, SsLocType::Structure, grp_id),
                    ),
                    _ => (),
                },
                SsModDomain::Ship => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(ship.fit_id, SsLocType::Ship, grp_id))
                    }
                    _ => (),
                },
                SsModDomain::Structure => match tgt_item {
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(structure.fit_id, SsLocType::Structure, grp_id),
                    ),
                    _ => (),
                },
                SsModDomain::Char => match tgt_item {
                    SsItem::Ship(ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(ship.fit_id, SsLocType::Character, grp_id),
                    ),
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(structure.fit_id, SsLocType::Character, grp_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            SsAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Everything | SsModDomain::Target => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(ship.fit_id, SsLocType::Ship, srq_id))
                    }
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(structure.fit_id, SsLocType::Structure, srq_id),
                    ),
                    _ => (),
                },
                SsModDomain::Ship => match tgt_item {
                    SsItem::Ship(ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(ship.fit_id, SsLocType::Ship, srq_id))
                    }
                    _ => (),
                },
                SsModDomain::Structure => match tgt_item {
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(structure.fit_id, SsLocType::Structure, srq_id),
                    ),
                    _ => (),
                },
                SsModDomain::Char => match tgt_item {
                    SsItem::Ship(ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(ship.fit_id, SsLocType::Character, srq_id),
                    ),
                    SsItem::Structure(structure) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(structure.fit_id, SsLocType::Character, srq_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            SsAffecteeFilter::OwnSrq(srq_id) => match tgt_item {
                SsItem::Ship(ship) => extend_vec_from_map_set_l1(affectees, &self.own_srq, &(ship.fit_id, srq_id)),
                SsItem::Structure(structure) => {
                    extend_vec_from_map_set_l1(affectees, &self.own_srq, &(structure.fit_id, srq_id))
                }
                _ => (),
            },
        }
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

fn get_fit_character(ss_view: &SsView, fit_id: &SsFitId) -> Option<SsItemId> {
    ss_view.fits.get_fit(fit_id).ok().map(|v| v.character).flatten()
}

fn check_domain_owner(dom: SsModDomain, fit: &SsFit) -> bool {
    match dom {
        SsModDomain::Char => fit.character.is_some(),
        SsModDomain::Ship => fit.ship.is_some(),
        SsModDomain::Structure => fit.structure.is_some(),
        _ => false,
    }
}
