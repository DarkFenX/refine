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
                    add_cmod(&mut self.cmods.direct, projectee_key, cmod, &mut self.cmods.by_aspec);
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
                                    &mut self.cmods.loc,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship),
                                    cmod,
                                    &mut self.cmods.by_aspec,
                                );
                                self.rmods_proj_active.add_entry(projectee_key, rmod);
                            }
                            Some(cmod)
                        }
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
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                Location::Target => match projectee_item {
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
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                Location::Target => match projectee_item {
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
                    remove_cmod(&mut self.cmods.direct, projectee_key, &cmod, &mut self.cmods.by_aspec);
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
                                &mut self.cmods.loc,
                                (projectee_ship.get_fit_key(), LocationKind::Ship),
                                &cmod,
                                &mut self.cmods.by_aspec,
                            );
                            self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                            Some(cmod)
                        }
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
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                Location::Target => match projectee_item {
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
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                Location::Target => match projectee_item {
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
}

pub(super) fn affectee_for_proj_target_reg(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) if let Location::Target = loc => {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            add_cmod(&mut cdata.direct, projectee_key, cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::Loc(loc)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind() {
                UShipKind::Ship => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                    add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
                    true
                }
                UShipKind::Structure => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                    add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocGrp(loc, item_grp_id)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind() {
                UShipKind::Ship => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                    add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
                    true
                }
                UShipKind::Structure => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                    add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocSrq(loc, srq_type_id)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind() {
                UShipKind::Ship => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                    add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                    true
                }
                UShipKind::Structure => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                    add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::OwnSrq(srq_type_id) if let UItem::Ship(projectee_ship) = projectee_item => {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), srq_type_id);
            add_cmod(&mut cdata.own_srq, key, cmod, &mut cdata.by_aspec);
            true
        }
        _ => false,
    }
}

pub(super) fn affectee_for_proj_target_unreg(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) if let Location::Target = loc => {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            remove_cmod(&mut cdata.direct, projectee_key, &cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::Loc(loc)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind() {
                UShipKind::Ship => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                    remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
                    true
                }
                UShipKind::Structure => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                    remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocGrp(loc, item_grp_id)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind() {
                UShipKind::Ship => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                    remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
                    true
                }
                UShipKind::Structure => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                    remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocSrq(loc, srq_type_id)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind() {
                UShipKind::Ship => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                    remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                    true
                }
                UShipKind::Structure => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                    remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::OwnSrq(srq_type_id) if let UItem::Ship(projectee_ship) = projectee_item => {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), srq_type_id);
            remove_cmod(&mut cdata.own_srq, key, &cmod, &mut cdata.by_aspec);
            true
        }
        _ => false,
    }
}
