use super::{add_cmod, remove_cmod};
use crate::{
    svc::calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    ud::{UItem, UItemKey, UShipKind},
};

impl StandardRegister {
    pub(super) fn proj_buff_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        self.process_buff_mod(rmod, projectee_key, projectee_item, true)
    }
    pub(super) fn query_buff_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        self.process_buff_mod(rmod, projectee_key, projectee_item, false)
    }
    fn process_buff_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
        register: bool,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => match projectee_item.is_buffable() {
                    true => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(&mut self.cmods_direct, projectee_key, cmod, &mut self.cmods_by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    false => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                },
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            if register {
                                add_cmod(&mut self.cmods_direct, projectee_key, cmod, &mut self.cmods_by_aspec);
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
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => match projectee_item {
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
                    _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => match projectee_item {
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
                    _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => match projectee_item {
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
                    _ => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                },
                _ => None,
            },
            _ => None,
        }
    }
    pub(super) fn unproj_buff_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => match projectee_item.is_buffable() {
                    true => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(&mut self.cmods_direct, &projectee_key, &cmod, &mut self.cmods_by_aspec);
                        self.rmods_proj_active.remove_entry(&projectee_key, &rmod);
                        Some(cmod)
                    }
                    false => self.unreg_inactive_proj_rmod(&rmod, &projectee_key),
                },
                Location::Ship => match projectee_item {
                    UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        UShipKind::Ship => {
                            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                            remove_cmod(&mut self.cmods_direct, &projectee_key, &cmod, &mut self.cmods_by_aspec);
                            self.rmods_proj_active.remove_entry(&projectee_key, &rmod);
                            Some(cmod)
                        }
                        _ => self.unreg_inactive_proj_rmod(&rmod, &projectee_key),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => match projectee_item {
                UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                    UShipKind::Ship => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods_loc,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(&projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => self.unreg_inactive_proj_rmod(&rmod, &projectee_key),
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => match projectee_item {
                UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                    UShipKind::Ship => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods_loc_grp,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(&projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => self.unreg_inactive_proj_rmod(&rmod, &projectee_key),
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => match projectee_item {
                UItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                    UShipKind::Ship => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods_loc_srq,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(&projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => self.unreg_inactive_proj_rmod(&rmod, &projectee_key),
                },
                _ => None,
            },
            _ => None,
        }
    }
    pub(super) fn reg_loc_root_for_proj_buff(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        self.process_buff_mod(rmod, projectee_key, projectee_item, true);
    }
    pub(super) fn unreg_loc_root_for_proj_buff(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    if projectee_item.is_buffable() {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(&mut self.cmods_direct, &projectee_key, &cmod, &mut self.cmods_by_aspec);
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                Location::Ship => {
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(&mut self.cmods_direct, &projectee_key, &cmod, &mut self.cmods_by_aspec);
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                if let UItem::Ship(projectee_ship) = projectee_item
                    && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_loc,
                        &(projectee_ship.get_fit_key(), LocationKind::Ship),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                }
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                if let UItem::Ship(projectee_ship) = projectee_item
                    && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_loc_grp,
                        &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                }
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                if let UItem::Ship(projectee_ship) = projectee_item
                    && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                {
                    let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_loc_srq,
                        &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                }
            }
            _ => (),
        }
    }
}
