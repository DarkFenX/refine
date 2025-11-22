use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier,
        registers::standard::{
            StandardRegister,
            func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemKey, UShipKind},
};

impl StandardRegister {
    pub(super) fn proj_target_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        self.process_target_mod(rmod, projectee_key, projectee_item, true)
    }
    pub(super) fn query_target_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        self.process_target_mod(rmod, projectee_key, projectee_item, false)
    }
    fn process_target_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
        register: bool,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Target => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    add_cmod(&mut self.cmods_direct, projectee_key, cmod, &mut self.cmods_by_aspec);
                    self.rmods_proj_active.add_entry(projectee_key, rmod);
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Target => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods_loc,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship),
                                    cmod,
                                    &mut self.cmods_by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods_loc,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure),
                                    cmod,
                                    &mut self.cmods_by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Target => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods_loc_grp,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                                    cmod,
                                    &mut self.cmods_by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods_loc_grp,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                                    cmod,
                                    &mut self.cmods_by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Target => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods_loc_srq,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                                    cmod,
                                    &mut self.cmods_by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods_loc_srq,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                                    cmod,
                                    &mut self.cmods_by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match projectee_item {
                UItem::Ship(projectee_ship) => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    if register {
                        add_cmod(
                            &mut self.cmods_own_srq,
                            (projectee_ship.get_fit_key(), srq_a_item_id),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                    }
                    Some(cmod)
                }
                _ => None,
            },
        }
    }
    pub(super) fn unproj_target_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Target => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(&mut self.cmods_direct, projectee_key, &cmod, &mut self.cmods_by_aspec);
                    self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Target => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_key(), LocationKind::Ship),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_key(), LocationKind::Structure),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Target => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Target => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match projectee_item {
                UItem::Ship(projectee_ship) => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_own_srq,
                        (projectee_ship.get_fit_key(), srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => None,
            },
        }
    }
    pub(super) fn reg_loc_root_for_proj_target(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        self.process_target_mod(rmod, projectee_key, projectee_item, true);
    }

    pub(super) fn unreg_loc_root_for_proj_target(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                if let Location::Target = loc {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(&mut self.cmods_direct, projectee_key, &cmod, &mut self.cmods_by_aspec);
                    self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                }
            }
            AffecteeFilter::Loc(loc) => {
                if let Location::Target = loc
                    && let UItem::Ship(projectee_ship) = projectee_item
                {
                    match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_key(), LocationKind::Ship),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_key(), LocationKind::Structure),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        }
                        _ => (),
                    }
                }
            }
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                if let Location::Target = loc
                    && let UItem::Ship(projectee_ship) = projectee_item
                {
                    match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        }
                        _ => (),
                    }
                }
            }
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                if let Location::Target = loc
                    && let UItem::Ship(projectee_ship) = projectee_item
                {
                    match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        }
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        }
                        _ => (),
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                if let UItem::Ship(projectee_ship) = projectee_item {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_own_srq,
                        (projectee_ship.get_fit_key(), srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                }
            }
        }
    }
}
