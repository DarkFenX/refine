use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier,
        registers::standard::{
            data::{StandardRegister, StandardRegisterCtxMods},
            func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemKey, UShipKind},
};

impl StandardRegister {
    pub(super) fn proj_system_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        self.process_system_mod(rmod, projectee_key, projectee_item, true)
    }
    pub(super) fn query_system_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        self.process_system_mod(rmod, projectee_key, projectee_item, false)
    }
    fn process_system_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
        register: bool,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.root,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.root,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(
                                &mut self.cmods.root,
                                (projectee_ship.get_fit_key(), LocationKind::Character),
                                cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.loc,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.loc,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(
                                &mut self.cmods.loc,
                                (projectee_ship.get_fit_key(), LocationKind::Character),
                                cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.loc_grp,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.loc_grp,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(
                                &mut self.cmods.loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id),
                                cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.loc_srq,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(
                                    &mut self.cmods.loc_srq,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
                        _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(
                                &mut self.cmods.loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id),
                                cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_type_id) => match projectee_item {
                UItem::Ship(projectee_ship) => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    if register {
                        add_cmod(
                            &mut self.cmods.own_srq,
                            (projectee_ship.get_fit_key(), srq_type_id),
                            cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                    }
                    Some(cmod)
                }
                _ => None,
            },
        }
    }
    pub(super) fn unproj_system_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.root,
                                (projectee_ship.get_fit_key(), LocationKind::Ship),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.root,
                                (projectee_ship.get_fit_key(), LocationKind::Structure),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.root,
                            (projectee_ship.get_fit_key(), LocationKind::Character),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.loc,
                                (projectee_ship.get_fit_key(), LocationKind::Ship),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.loc,
                                (projectee_ship.get_fit_key(), LocationKind::Structure),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc,
                            (projectee_ship.get_fit_key(), LocationKind::Character),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_grp,
                            (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Structure => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(
                                &mut self.cmods.loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                    },
                    _ => None,
                },
                Location::Char => match projectee_item {
                    UItem::Ship(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_srq,
                            (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_type_id) => match projectee_item {
                UItem::Ship(projectee_ship) => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods.own_srq,
                        (projectee_ship.get_fit_key(), srq_type_id),
                        &cmod,
                        &mut self.cmods.by_aspec,
                    );
                    self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => None,
            },
        }
    }
    pub(super) fn reg_loc_root_for_proj_system(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        self.process_system_mod(rmod, projectee_key, projectee_item, true);
    }
    pub(super) fn unreg_loc_root_for_proj_system(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Ship => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.root,
                            (projectee_ship.get_fit_key(), LocationKind::Ship),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Structure => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.root,
                            (projectee_ship.get_fit_key(), LocationKind::Structure),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Char => {
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.root,
                            (projectee_ship.get_fit_key(), LocationKind::Character),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Ship => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc,
                            (projectee_ship.get_fit_key(), LocationKind::Ship),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Structure => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc,
                            (projectee_ship.get_fit_key(), LocationKind::Structure),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Char => {
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc,
                            (projectee_ship.get_fit_key(), LocationKind::Character),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                Location::Ship => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_grp,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Structure => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_grp,
                            (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Char => {
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_grp,
                            (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                Location::Ship => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_srq,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Structure => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_srq,
                            (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Char => {
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods.loc_srq,
                            (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
                        );
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                _ => (),
            },
            AffecteeFilter::OwnSrq(srq_type_id) => {
                if let UItem::Ship(projectee_ship) = projectee_item {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods.own_srq,
                        (projectee_ship.get_fit_key(), srq_type_id),
                        &cmod,
                        &mut self.cmods.by_aspec,
                    );
                    self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                }
            }
        }
    }
}

pub(super) fn affectee_for_proj_system_reg(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                add_cmod(&mut cdata.root, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                add_cmod(&mut cdata.root, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                add_cmod(&mut cdata.root, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::Loc(loc) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id);
                add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id);
                add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::OwnSrq(srq_type_id) if let UItem::Ship(projectee_ship) = projectee_item => {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), srq_type_id);
            add_cmod(&mut cdata.own_srq, key, cmod, &mut cdata.by_aspec);
            true
        }
        _ => false,
    }
}

pub(super) fn affectee_for_proj_system_unreg(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                remove_cmod(&mut cdata.root, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                remove_cmod(&mut cdata.root, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                remove_cmod(&mut cdata.root, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::Loc(loc) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id);
                remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
            Location::Ship
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Ship = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let UShipKind::Structure = projectee_ship.get_kind() =>
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Char if let UItem::Ship(projectee_ship) = projectee_item => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id);
                remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::OwnSrq(srq_type_id) if let UItem::Ship(projectee_ship) = projectee_item => {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), srq_type_id);
            remove_cmod(&mut cdata.own_srq, key, &cmod, &mut cdata.by_aspec);
            true
        }
        _ => false,
    }
}
