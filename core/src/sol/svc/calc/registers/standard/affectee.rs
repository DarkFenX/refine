use crate::{
    sol::{
        FitId, ItemId,
        svc::calc::{AffecteeFilter, Context, CtxModifier, Location, LocationKind, ModifierKind},
        uad::{
            Uad,
            fit::Fit,
            item::{Item, ShipKind},
        },
    },
    util::extend_vec_from_map_set_l1,
};

use super::{PotentialLocations, StandardRegister};

impl StandardRegister {
    // Query methods
    pub(in crate::sol::svc::calc) fn fill_affectees(
        &self,
        affectees: &mut Vec<ItemId>,
        uad: &Uad,
        modifier: &CtxModifier,
    ) {
        affectees.clear();
        match modifier.ctx {
            Context::None => self.fill_affectees_no_context(affectees, uad, modifier),
            Context::Fit(fit_id) => self.fill_affectees_for_fit(affectees, uad, modifier, fit_id),
            Context::Item(item_id) => match modifier.raw.kind {
                ModifierKind::System => self.fill_affectees_for_item_system(affectees, uad, modifier, item_id),
                ModifierKind::Targeted => self.fill_affectees_for_item_target(affectees, uad, modifier, item_id),
                ModifierKind::Buff => self.fill_affectees_for_item_buff(affectees, uad, modifier, item_id),
                _ => (),
            },
        }
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn reg_affectee(&mut self, uad: &Uad, item: &Item) {
        let item_id = item.get_item_id();
        let fit = item.get_fit_id().and_then(|v| uad.fits.get_fit(&v).ok());
        let root_loc = item.get_root_loc_kind();
        let a_item_grp_id = item.get_a_group_id();
        let a_srqs = item.get_a_skill_reqs();
        if let (Some(fit), Some(root_loc)) = (fit, root_loc) {
            self.affectee_root.add_entry((fit.id, root_loc), item_id);
        }
        if let Some(fit) = fit {
            for loc in PotentialLocations::new(item) {
                self.affectee_loc.add_entry((fit.id, loc), item_id);
            }
        }
        if let (Some(fit), Some(a_item_grp_id)) = (fit, a_item_grp_id) {
            for loc in PotentialLocations::new(item) {
                self.affectee_loc_grp.add_entry((fit.id, loc, a_item_grp_id), item_id);
            }
        }
        if let (Some(fit), Some(a_srqs)) = (fit, &a_srqs) {
            for loc in PotentialLocations::new(item) {
                for srq_a_item_id in a_srqs.keys() {
                    self.affectee_loc_srq.add_entry((fit.id, loc, *srq_a_item_id), item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(a_srqs)) = (fit, &a_srqs) {
                for srq_a_item_id in a_srqs.keys() {
                    self.affectee_own_srq.add_entry((fit.id, *srq_a_item_id), item_id);
                }
            }
        }
        if item.is_buffable() {
            if let Some(fit) = fit {
                self.affectee_buffable.add_entry(fit.id, item_id);
            }
            self.reg_buffable_for_sw(item);
            self.reg_buffable_for_fw(item);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_affectee(&mut self, uad: &Uad, item: &Item) {
        let item_id = item.get_item_id();
        let fit = item.get_fit_id().and_then(|v| uad.fits.get_fit(&v).ok());
        let root_loc = item.get_root_loc_kind();
        let a_item_grp_id = item.get_a_group_id();
        let a_srqs = item.get_a_skill_reqs();
        if let (Some(fit), Some(root_loc)) = (fit, root_loc) {
            self.affectee_root.remove_entry(&(fit.id, root_loc), &item_id);
        }
        if let Some(fit) = fit {
            for loc in PotentialLocations::new(item) {
                self.affectee_loc.remove_entry(&(fit.id, loc), &item_id);
            }
        }
        if let (Some(fit), Some(a_item_grp_id)) = (fit, a_item_grp_id) {
            for loc in PotentialLocations::new(item) {
                self.affectee_loc_grp
                    .remove_entry(&(fit.id, loc, a_item_grp_id), &item_id);
            }
        }
        if let (Some(fit), Some(a_srqs)) = (fit, &a_srqs) {
            for loc in PotentialLocations::new(item) {
                for srq_a_item_id in a_srqs.keys() {
                    self.affectee_loc_srq
                        .remove_entry(&(fit.id, loc, *srq_a_item_id), &item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(a_srqs)) = (fit, &a_srqs) {
                for srq_a_item_id in a_srqs.keys() {
                    self.affectee_own_srq.remove_entry(&(fit.id, *srq_a_item_id), &item_id);
                }
            }
        }
        if item.is_buffable() {
            if let Some(fit) = fit {
                self.affectee_buffable.remove_entry(&fit.id, &item_id);
            }
            self.unreg_buffable_for_sw(item);
            self.unreg_buffable_for_fw(item);
        }
    }
    // Private methods
    fn fill_affectees_no_context(&self, affectees: &mut Vec<ItemId>, uad: &Uad, modifier: &CtxModifier) {
        if let AffecteeFilter::Direct(loc) = modifier.raw.affectee_filter {
            match loc {
                Location::Item => {
                    affectees.push(modifier.raw.affector_item_id);
                }
                Location::Other => {
                    let item = uad.items.get_by_id(&modifier.raw.affector_item_id).unwrap();
                    if let Some(other_item_id) = item.get_other() {
                        affectees.push(other_item_id);
                    }
                }
                _ => (),
            }
        }
    }
    fn fill_affectees_for_fit(&self, affectees: &mut Vec<ItemId>, uad: &Uad, modifier: &CtxModifier, fit_id: FitId) {
        match modifier.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => extend_vec_from_map_set_l1(affectees, &self.affectee_buffable, &fit_id),
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_root, &(fit_id, loc_kind));
                        }
                    }
                }
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Everything => {
                    if is_fit_of_ship_kind(uad, &fit_id) {
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &(fit_id, LocationKind::Ship))
                    }
                }
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &(fit_id, loc_kind));
                        }
                    }
                }
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Everything => {
                    if is_fit_of_ship_kind(uad, &fit_id) {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(fit_id, LocationKind::Ship, a_item_grp_id),
                        );
                    }
                }
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(fit_id, loc_kind, a_item_grp_id),
                            );
                        }
                    }
                }
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Everything => {
                    if is_fit_of_ship_kind(uad, &fit_id) {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(fit_id, LocationKind::Ship, srq_a_item_id),
                        );
                    }
                }
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = uad.fits.get_fit(&fit_id).unwrap();
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(fit_id, loc_kind, srq_a_item_id),
                            );
                        }
                    }
                }
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &(fit_id, srq_a_item_id));
            }
        }
    }
    fn fill_affectees_for_item_system(
        &self,
        affectees: &mut Vec<ItemId>,
        uad: &Uad,
        modifier: &CtxModifier,
        projectee_item_id: ItemId,
    ) {
        match modifier.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            affectees.push(projectee_ship.get_item_id())
                        }
                    }
                }
                Location::Structure => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Structure) {
                            affectees.push(projectee_ship.get_item_id())
                        }
                    }
                }
                Location::Char => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if let Some(char_id) = get_fit_character(uad, &projectee_ship.get_fit_id()) {
                            affectees.push(char_id);
                        }
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship),
                            )
                        }
                    }
                }
                Location::Structure => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Structure) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure),
                            )
                        }
                    }
                }
                Location::Char => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc,
                            &(projectee_ship.get_fit_id(), LocationKind::Character),
                        )
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                            );
                        }
                    }
                }
                Location::Structure => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Structure) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure, a_item_grp_id),
                            );
                        }
                    }
                }
                Location::Char => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(projectee_ship.get_fit_id(), LocationKind::Character, a_item_grp_id),
                        );
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                            )
                        }
                    }
                }
                Location::Structure => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Structure) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure, srq_a_item_id),
                            )
                        }
                    }
                }
                Location::Char => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(projectee_ship.get_fit_id(), LocationKind::Character, srq_a_item_id),
                        )
                    }
                }
                _ => (),
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                if let Item::Ship(projectee_ship) = projectee_item {
                    extend_vec_from_map_set_l1(
                        affectees,
                        &self.affectee_own_srq,
                        &(projectee_ship.get_fit_id(), srq_a_item_id),
                    )
                }
            }
        }
    }
    fn fill_affectees_for_item_target(
        &self,
        affectees: &mut Vec<ItemId>,
        uad: &Uad,
        modifier: &CtxModifier,
        projectee_item_id: ItemId,
    ) {
        match modifier.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                if matches!(loc, Location::Target) {
                    affectees.push(projectee_item_id)
                }
            }
            AffecteeFilter::Loc(loc) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship),
                            ),
                            ShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                            ),
                            ShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure, a_item_grp_id),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                            ),
                            ShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure, srq_a_item_id),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                if let Item::Ship(projectee_ship) = projectee_item {
                    extend_vec_from_map_set_l1(
                        affectees,
                        &self.affectee_own_srq,
                        &(projectee_ship.get_fit_id(), srq_a_item_id),
                    );
                }
            }
        }
    }
    fn fill_affectees_for_item_buff(
        &self,
        affectees: &mut Vec<ItemId>,
        uad: &Uad,
        modifier: &CtxModifier,
        projectee_item_id: ItemId,
    ) {
        match modifier.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if projectee_item.is_buffable() {
                        affectees.push(projectee_item_id)
                    }
                }
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            affectees.push(projectee_ship.get_item_id())
                        }
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Everything => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship),
                            );
                        }
                    }
                }
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship),
                            );
                        }
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Everything => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                            );
                        }
                    }
                }
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                            );
                        }
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Everything => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                            );
                        }
                    }
                }
                Location::Ship => {
                    let projectee_item = uad.items.get_by_id(&projectee_item_id).unwrap();
                    if let Item::Ship(projectee_ship) = projectee_item {
                        if matches!(projectee_ship.get_kind(), ShipKind::Ship) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
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

fn get_fit_character(uad: &Uad, fit_id: &FitId) -> Option<ItemId> {
    uad.fits.get_fit(fit_id).ok().and_then(|v| v.character)
}

fn check_loc_owner(loc: Location, fit: &Fit) -> bool {
    match loc {
        Location::Char => fit.character.is_some(),
        Location::Ship => matches!(fit.kind, ShipKind::Ship),
        Location::Structure => matches!(fit.kind, ShipKind::Structure),
        _ => false,
    }
}

fn is_fit_of_ship_kind(uad: &Uad, fit_id: &FitId) -> bool {
    let fit = uad.fits.get_fit(fit_id).unwrap();
    matches!(fit.kind, ShipKind::Ship)
}
