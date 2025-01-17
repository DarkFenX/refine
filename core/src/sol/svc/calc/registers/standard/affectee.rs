use crate::{
    defs::{SolFitId, SolItemId},
    sol::{
        svc::calc::{SolAffecteeFilter, SolContext, SolCtxModifier, SolDomain, SolLocationKind, SolModifierKind},
        uad::{
            fit::SolFit,
            item::{SolItem, SolShipKind},
            SolUad,
        },
    },
    util::extend_vec_from_map_set_l1,
};

use super::{SolPotentialLocations, SolStandardRegister};

impl SolStandardRegister {
    // Query methods
    pub(in crate::sol::svc::calc) fn fill_affectees(
        &self,
        affectees: &mut Vec<SolItemId>,
        uad: &SolUad,
        modifier: &SolCtxModifier,
    ) {
        affectees.clear();
        match modifier.ctx {
            SolContext::None => self.fill_affectees_no_context(affectees, uad, modifier),
            SolContext::Fit(fit_id) => self.fill_affectees_for_fit(affectees, uad, modifier, fit_id),
            SolContext::Item(item_id) => match modifier.raw.kind {
                SolModifierKind::System => self.fill_affectees_for_item_system(affectees, uad, modifier, item_id),
                SolModifierKind::Targeted => self.fill_affectees_for_item_target(affectees, uad, modifier, item_id),
                SolModifierKind::Buff => self.fill_affectees_for_item_buff(affectees, uad, modifier, item_id),
                _ => (),
            },
        }
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn reg_affectee(&mut self, uad: &SolUad, item: &SolItem) {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| uad.fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_kind();
        let grp_id_opt = item.get_group_id();
        let srqs_opt = item.get_skill_reqs();
        if let (Some(fit), Some(root_loc)) = (fit_opt, root_loc_opt) {
            self.affectee_root.add_entry((fit.id, root_loc), item_id);
        }
        if let Some(fit) = fit_opt {
            for loc in SolPotentialLocations::new(item) {
                self.affectee_loc.add_entry((fit.id, loc), item_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc in SolPotentialLocations::new(item) {
                self.affectee_loc_grp.add_entry((fit.id, loc, grp_id), item_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc in SolPotentialLocations::new(item) {
                for srq_id in srqs.keys() {
                    self.affectee_loc_srq.add_entry((fit.id, loc, *srq_id), item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for skill_type_id in srqs.keys() {
                    self.affectee_own_srq.add_entry((fit.id, *skill_type_id), item_id);
                }
            }
        }
        if item.is_buffable() {
            if let Some(fit) = fit_opt {
                self.affectee_buffable.add_entry(fit.id, item_id);
            }
            self.reg_buffable_for_sw(item);
            self.reg_buffable_for_fw(item);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_affectee(&mut self, uad: &SolUad, item: &SolItem) {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| uad.fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_kind();
        let grp_id_opt = item.get_group_id();
        let srqs_opt = item.get_skill_reqs();
        if let (Some(fit), Some(root_loc)) = (fit_opt, root_loc_opt) {
            self.affectee_root.remove_entry(&(fit.id, root_loc), &item_id);
        }
        if let Some(fit) = fit_opt {
            for loc in SolPotentialLocations::new(item) {
                self.affectee_loc.remove_entry(&(fit.id, loc), &item_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc in SolPotentialLocations::new(item) {
                self.affectee_loc_grp.remove_entry(&(fit.id, loc, grp_id), &item_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc in SolPotentialLocations::new(item) {
                for srq_id in srqs.keys() {
                    self.affectee_loc_srq.remove_entry(&(fit.id, loc, *srq_id), &item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for srq_id in srqs.keys() {
                    self.affectee_own_srq.remove_entry(&(fit.id, *srq_id), &item_id);
                }
            }
        }
        if item.is_buffable() {
            if let Some(fit) = fit_opt {
                self.affectee_buffable.remove_entry(&fit.id, &item_id);
            }
            self.unreg_buffable_for_sw(item);
            self.unreg_buffable_for_fw(item);
        }
    }
    // Private methods
    fn fill_affectees_no_context(&self, affectees: &mut Vec<SolItemId>, uad: &SolUad, modifier: &SolCtxModifier) {
        if let SolAffecteeFilter::Direct(dom) = modifier.raw.affectee_filter {
            match dom {
                SolDomain::Item => {
                    affectees.push(modifier.raw.affector_item_id);
                }
                SolDomain::Other => {
                    let item = uad.items.get_item(&modifier.raw.affector_item_id).unwrap();
                    if let Some(other_item_id) = item.get_other() {
                        affectees.push(other_item_id);
                    }
                }
                _ => (),
            }
        }
    }
    fn fill_affectees_for_fit(
        &self,
        affectees: &mut Vec<SolItemId>,
        uad: &SolUad,
        modifier: &SolCtxModifier,
        fit_id: SolFitId,
    ) {
        match modifier.raw.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => extend_vec_from_map_set_l1(affectees, &self.affectee_buffable, &fit_id),
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_root, &(fit_id, loc));
                        }
                    }
                }
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => {
                    if is_fit_of_ship_kind(uad, &fit_id) {
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &(fit_id, SolLocationKind::Ship))
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &(fit_id, loc));
                        }
                    }
                }
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => {
                    if is_fit_of_ship_kind(uad, &fit_id) {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(fit_id, SolLocationKind::Ship, grp_id),
                        );
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &(fit_id, loc, grp_id));
                        }
                    }
                }
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => {
                    if is_fit_of_ship_kind(uad, &fit_id) {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(fit_id, SolLocationKind::Ship, srq_id),
                        );
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &(fit_id, loc, srq_id));
                        }
                    }
                }
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &(fit_id, srq_id));
            }
        }
    }
    fn fill_affectees_for_item_system(
        &self,
        affectees: &mut Vec<SolItemId>,
        uad: &SolUad,
        modifier: &SolCtxModifier,
        projectee_item_id: SolItemId,
    ) {
        match modifier.raw.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            affectees.push(projectee_ship.get_id())
                        }
                    }
                }
                SolDomain::Structure => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Structure) {
                            affectees.push(projectee_ship.get_id())
                        }
                    }
                }
                SolDomain::Char => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if let Some(char_id) = get_fit_character(uad, &projectee_ship.get_fit_id()) {
                            affectees.push(char_id);
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            )
                        }
                    }
                }
                SolDomain::Structure => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Structure) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure),
                            )
                        }
                    }
                }
                SolDomain::Char => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Character),
                        )
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                            );
                        }
                    }
                }
                SolDomain::Structure => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Structure) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure, grp_id),
                            );
                        }
                    }
                }
                SolDomain::Char => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Character, grp_id),
                        );
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                            )
                        }
                    }
                }
                SolDomain::Structure => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Structure) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure, srq_id),
                            )
                        }
                    }
                }
                SolDomain::Char => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Character, srq_id),
                        )
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                if let SolItem::Ship(projectee_ship) = projectee_item {
                    extend_vec_from_map_set_l1(
                        affectees,
                        &self.affectee_own_srq,
                        &(projectee_ship.get_fit_id(), srq_id),
                    )
                }
            }
        }
    }
    fn fill_affectees_for_item_target(
        &self,
        affectees: &mut Vec<SolItemId>,
        uad: &SolUad,
        modifier: &SolCtxModifier,
        projectee_item_id: SolItemId,
    ) {
        match modifier.raw.affectee_filter {
            SolAffecteeFilter::Direct(dom) => {
                if matches!(dom, SolDomain::Target) {
                    affectees.push(projectee_item_id)
                }
            }
            SolAffecteeFilter::Loc(dom) => {
                if matches!(dom, SolDomain::Target) {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            SolShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            ),
                            SolShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            SolAffecteeFilter::LocGrp(dom, grp_id) => {
                if matches!(dom, SolDomain::Target) {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            SolShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                            ),
                            SolShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure, grp_id),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            SolAffecteeFilter::LocSrq(dom, srq_id) => {
                if matches!(dom, SolDomain::Target) {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            SolShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                            ),
                            SolShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure, srq_id),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            SolAffecteeFilter::OwnSrq(srq_id) => {
                let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                if let SolItem::Ship(projectee_ship) = projectee_item {
                    extend_vec_from_map_set_l1(
                        affectees,
                        &self.affectee_own_srq,
                        &(projectee_ship.get_fit_id(), srq_id),
                    );
                }
            }
        }
    }
    fn fill_affectees_for_item_buff(
        &self,
        affectees: &mut Vec<SolItemId>,
        uad: &SolUad,
        modifier: &SolCtxModifier,
        projectee_item_id: SolItemId,
    ) {
        match modifier.raw.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if projectee_item.is_buffable() {
                        affectees.push(projectee_item_id)
                    }
                }
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            affectees.push(projectee_ship.get_id())
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            );
                        }
                    }
                }
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            );
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                            );
                        }
                    }
                }
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                            );
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                            );
                        }
                    }
                }
                SolDomain::Ship => {
                    let projectee_item = uad.items.get_item(&projectee_item_id).unwrap();
                    if let SolItem::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), SolShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                            );
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn get_fit_character(uad: &SolUad, fit_id: &SolFitId) -> Option<SolItemId> {
    uad.fits.get_fit(fit_id).ok().map(|v| v.character).flatten()
}

fn check_domain_owner(dom: SolDomain, fit: &SolFit) -> bool {
    match dom {
        SolDomain::Char => fit.character.is_some(),
        SolDomain::Ship => matches!(fit.kind, SolShipKind::Ship),
        SolDomain::Structure => matches!(fit.kind, SolShipKind::Structure),
        _ => false,
    }
}

fn is_fit_of_ship_kind(uad: &SolUad, fit_id: &SolFitId) -> bool {
    let fit = uad.fits.get_fit(&fit_id).unwrap();
    matches!(fit.kind, SolShipKind::Ship)
}
