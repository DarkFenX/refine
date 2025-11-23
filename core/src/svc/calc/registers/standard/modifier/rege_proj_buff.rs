use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier,
        registers::standard::{
            data::{StandardRegister, StandardRegisterCtxMods},
            func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemKey},
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
                            add_cmod(&mut self.cmods.direct, projectee_key, cmod, &mut self.cmods.by_aspec);
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
                                &mut self.cmods.loc,
                                (projectee_ship.get_fit_key(), LocationKind::Ship),
                                cmod,
                                &mut self.cmods.by_aspec,
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
                                &mut self.cmods.loc_grp,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                                cmod,
                                &mut self.cmods.by_aspec,
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
                                &mut self.cmods.loc_srq,
                                (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                                cmod,
                                &mut self.cmods.by_aspec,
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
                        remove_cmod(&mut self.cmods.direct, projectee_key, &cmod, &mut self.cmods.by_aspec);
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
                            &mut self.cmods.loc,
                            (projectee_ship.get_fit_key(), LocationKind::Ship),
                            &cmod,
                            &mut self.cmods.by_aspec,
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
                            &mut self.cmods.loc_grp,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
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
                            &mut self.cmods.loc_srq,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id),
                            &cmod,
                            &mut self.cmods.by_aspec,
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions which are called when already projectee item is loaded/unloaded. Only modifiers which
// depend on projectee item properties should be processed by those functions.
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn load_affectee_for_proj_buff(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_id))
            if let Some(buffable_item_lists) = projectee_item.get_item_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            add_cmod(&mut cdata.direct, projectee_key, cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
            add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
            add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
            add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
            true
        }
        _ => false,
    }
}
pub(super) fn unload_affectee_for_proj_buff(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_id))
            if let Some(buffable_item_lists) = projectee_item.get_item_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            remove_cmod(&mut cdata.direct, projectee_key, &cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
            remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
            remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::from_raw_with_item(*rmod, projectee_key);
            let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
            remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
            true
        }
        _ => false,
    }
}
