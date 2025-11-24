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
        {
            let projectee_ship = match projectee_item {
                UItem::Ship(projectee_ship) => projectee_ship,
                _ => return None,
            };
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc) => match loc {
                    Location::Ship => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                            add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Structure => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                            add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Char => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                        add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                AffecteeFilter::Loc(loc) => match loc {
                    Location::Ship => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                            add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Structure => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                            add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Char => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                        add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                    Location::Ship => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                            add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Structure => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                            add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Char => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id);
                        add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                    Location::Ship => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                            add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Structure => match projectee_ship.get_kind() {
                        UShipKind::Structure => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                            add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                            Some(cmod)
                        }
                        _ => {
                            self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                            None
                        }
                    },
                    Location::Char => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id);
                        add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    _ => None,
                },
                AffecteeFilter::OwnSrq(srq_type_id) => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), srq_type_id);
                    add_cmod(&mut self.cmods.own_srq, key, cmod, &mut self.cmods.by_aspec);
                    self.rmods_proj_active.add_entry(projectee_key, rmod);
                    Some(cmod)
                }
            }
        }
    }
    pub(super) fn unproj_system_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        // Modifiers passed to this method were not validated, so for every valid configuration we
        // have to remove a modifier from appropriate raw modifier container
        let projectee_ship = match projectee_item {
            UItem::Ship(projectee_ship) => projectee_ship,
            _ => return None,
        };
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Ship => match projectee_ship.get_kind() {
                    UShipKind::Ship => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                        remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Structure => match projectee_ship.get_kind() {
                    UShipKind::Structure => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                        remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Char => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                    remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                    self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Ship => match projectee_ship.get_kind() {
                    UShipKind::Ship => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                        remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Structure => match projectee_ship.get_kind() {
                    UShipKind::Structure => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                        remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Char => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                    self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                Location::Ship => match projectee_ship.get_kind() {
                    UShipKind::Ship => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                        remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Structure => match projectee_ship.get_kind() {
                    UShipKind::Structure => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                        remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Char => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                    self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                Location::Ship => match projectee_ship.get_kind() {
                    UShipKind::Ship => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                        remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Structure => match projectee_ship.get_kind() {
                    UShipKind::Structure => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                        remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                },
                Location::Char => {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                    self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), srq_type_id);
                remove_cmod(&mut self.cmods.own_srq, key, &cmod, &mut self.cmods.by_aspec);
                self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                Some(cmod)
            }
        }
    }
    pub(super) fn query_system_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        {
            let projectee_ship = match projectee_item {
                UItem::Ship(projectee_ship) => projectee_ship,
                _ => return None,
            };
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc)
                | AffecteeFilter::Loc(loc)
                | AffecteeFilter::LocGrp(loc, _)
                | AffecteeFilter::LocSrq(loc, _) => match loc {
                    Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                        Some(CtxModifier::from_raw_with_item(rmod, projectee_key))
                    }
                    Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                        Some(CtxModifier::from_raw_with_item(rmod, projectee_key))
                    }
                    Location::Char => Some(CtxModifier::from_raw_with_item(rmod, projectee_key)),
                    _ => None,
                },
                AffecteeFilter::OwnSrq(_) => Some(CtxModifier::from_raw_with_item(rmod, projectee_key)),
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions which are called when already projectee item is loaded/unloaded. Only modifiers which
// depend on projectee item properties should be processed by those functions.
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn load_affectee_for_proj_system(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    let projectee_ship = match projectee_item {
        UItem::Ship(projectee_ship) => projectee_ship,
        _ => return false,
    };
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                add_cmod(&mut cdata.root, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                add_cmod(&mut cdata.root, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::Loc(loc) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        _ => false,
    }
}

pub(super) fn unload_affectee_for_proj_system(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    let projectee_ship = match projectee_item {
        UItem::Ship(projectee_ship) => projectee_ship,
        _ => return false,
    };
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                remove_cmod(&mut cdata.root, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                remove_cmod(&mut cdata.root, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::Loc(loc) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
            Location::Ship if let UShipKind::Ship = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                true
            }
            Location::Structure if let UShipKind::Structure = projectee_ship.get_kind() => {
                let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                true
            }
            _ => false,
        },
        _ => false,
    }
}
