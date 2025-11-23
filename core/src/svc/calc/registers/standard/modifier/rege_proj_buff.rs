use crate::{
    ad::AItemListId,
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier,
        registers::standard::{
            StandardRegister,
            func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemKey, UShip},
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
    // TODO: check if ther needs to be any complex logic, maybe can use active container
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
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                match projectee_item.is_item_buffable_by_item_list(&item_list_id) {
                    true => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(&mut self.cmods_direct, projectee_key, cmod, &mut self.cmods_by_aspec);
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    false => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
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
                    None => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                                cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    None => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        if register {
                            add_cmod(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                                cmod,
                                &mut self.cmods_by_aspec,
                            );
                            self.rmods_proj_active.add_entry(projectee_key, rmod);
                        }
                        Some(cmod)
                    }
                    None => self.reg_inactive_proj_rmod(rmod, projectee_key, register),
                }
            }
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
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                match projectee_item.is_item_buffable_by_item_list(&item_list_id) {
                    true => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(&mut self.cmods_direct, projectee_key, &cmod, &mut self.cmods_by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    false => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
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
                    None => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods_loc_grp,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    None => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
                        remove_cmod(
                            &mut self.cmods_loc_srq,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    None => self.unreg_inactive_proj_rmod(&rmod, projectee_key),
                }
            }
            _ => None,
        }
    }
    pub(in crate::svc::calc::registers::standard) fn reg_affectee_for_proj_buff(
        &mut self,
        projectee_key: UItemKey,
        projectee_ship: Option<&UShip>,
        buffable_item_lists: &[AItemListId],
    ) {
        // Direct changes can affect all buffable items
        self.rmods_proj_inactive
            .buffer_if(projectee_key, |r| match r.affectee_filter {
                AffecteeFilter::Direct(Location::ItemList(item_list_id)) => buffable_item_lists.contains(&item_list_id),
                _ => false,
            });
        for &rmod in self.rmods_proj_inactive.iter_buffer() {
            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
            add_cmod(&mut self.cmods_direct, projectee_key, cmod, &mut self.cmods_by_aspec);
        }
        self.rmods_proj_active
            .extend_entries(projectee_key, self.rmods_proj_inactive.drain_buffer());
        // Indirect changes can be applied only via ships
        let projectee_ship = match projectee_ship {
            Some(projectee_ship) => projectee_ship,
            None => return,
        };
        self.rmods_proj_inactive
            .buffer_if(projectee_key, |r| match r.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_id)) => buffable_item_lists.contains(&item_list_id),
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), _) => {
                    buffable_item_lists.contains(&item_list_id)
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), _) => {
                    buffable_item_lists.contains(&item_list_id)
                }
                _ => false,
            });
        for rmod in self.rmods_proj_inactive.iter_buffer() {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(_) => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    add_cmod(
                        &mut self.cmods_loc,
                        (projectee_ship.get_fit_key(), LocationKind::Ship),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocGrp(_, item_grp_id) => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    add_cmod(
                        &mut self.cmods_loc_grp,
                        (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocSrq(_, srq_type_id) => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    add_cmod(
                        &mut self.cmods_loc_srq,
                        (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                _ => (),
            }
        }
        self.rmods_proj_active
            .extend_entries(projectee_key, self.rmods_proj_inactive.drain_buffer());
    }
    pub(in crate::svc::calc::registers::standard) fn unreg_affectee_for_proj_buff(
        &mut self,
        projectee_key: UItemKey,
        projectee_ship: Option<&UShip>,
        buffable_item_lists: &[AItemListId],
    ) {
        // Direct changes can affect all buffable items
        self.rmods_proj_active
            .buffer_if(projectee_key, |r| match r.affectee_filter {
                AffecteeFilter::Direct(Location::ItemList(item_list_id)) => buffable_item_lists.contains(&item_list_id),
                _ => false,
            });
        for &rmod in self.rmods_proj_active.iter_buffer() {
            let cmod = CtxModifier::from_raw_with_item(rmod, projectee_key);
            remove_cmod(&mut self.cmods_direct, projectee_key, &cmod, &mut self.cmods_by_aspec);
        }
        self.rmods_proj_inactive
            .extend_entries(projectee_key, self.rmods_proj_active.drain_buffer());
        // Indirect changes can be applied only via ships
        let projectee_ship = match projectee_ship {
            Some(projectee_ship) => projectee_ship,
            None => return,
        };
        self.rmods_proj_active
            .buffer_if(projectee_key, |r| match r.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_id)) => buffable_item_lists.contains(&item_list_id),
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), _) => {
                    buffable_item_lists.contains(&item_list_id)
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), _) => {
                    buffable_item_lists.contains(&item_list_id)
                }
                _ => false,
            });
        for rmod in self.rmods_proj_active.iter_buffer() {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(_) => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_loc,
                        (projectee_ship.get_fit_key(), LocationKind::Ship),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocGrp(_, item_grp_id) => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_loc_grp,
                        (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocSrq(_, srq_type_id) => {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
                    remove_cmod(
                        &mut self.cmods_loc_srq,
                        (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                _ => (),
            }
        }
        self.rmods_proj_inactive
            .extend_entries(projectee_key, self.rmods_proj_active.drain_buffer());
    }
}
