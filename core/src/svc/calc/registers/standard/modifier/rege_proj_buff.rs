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
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                match projectee_item.is_item_buffable_by_item_list(&item_list_id) {
                    true => {
                        let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
                        add_cmod(&mut self.cmods.direct, projectee_key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    false => {
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        None
                    }
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, LocationKind::Ship);
                        add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    None => {
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, LocationKind::Ship, item_grp_id);
                        add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    None => {
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, LocationKind::Ship, srq_type_id);
                        add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    None => {
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        None
                    }
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
        // Modifiers passed to this method were not validated, so for every valid configuration we
        // have to remove a modifier from appropriate raw modifier container
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                match projectee_item.is_item_buffable_by_item_list(&item_list_id) {
                    true => {
                        let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
                        remove_cmod(&mut self.cmods.direct, projectee_key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    false => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, LocationKind::Ship);
                        remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    None => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, LocationKind::Ship, item_grp_id);
                        remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    None => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    Some(projectee_ship) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, LocationKind::Ship, srq_type_id);
                        remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    None => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                }
            }
            _ => None,
        }
    }
    pub(super) fn query_buff_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id))
                if projectee_item.is_item_buffable_by_item_list(&item_list_id) =>
            {
                let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
                Some(cmod)
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id))
            | AffecteeFilter::LocGrp(Location::ItemList(item_list_id), _)
            | AffecteeFilter::LocSrq(Location::ItemList(item_list_id), _)
                if let Some(projectee_ship) = projectee_item.is_ship_buffable_by_item_list(&item_list_id) =>
            {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                Some(cmod)
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
            let cmod = CtxModifier::new_with_projectee_item(*rmod, projectee_key);
            add_cmod(&mut cdata.direct, projectee_key, cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, LocationKind::Ship);
            add_cmod(&mut cdata.loc, key, cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, LocationKind::Ship, item_grp_id);
            add_cmod(&mut cdata.loc_grp, key, cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, LocationKind::Ship, srq_type_id);
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
            let cmod = CtxModifier::new_with_projectee_item(*rmod, projectee_key);
            remove_cmod(&mut cdata.direct, projectee_key, &cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, LocationKind::Ship);
            remove_cmod(&mut cdata.loc, key, &cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, LocationKind::Ship, item_grp_id);
            remove_cmod(&mut cdata.loc_grp, key, &cmod, &mut cdata.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, LocationKind::Ship, srq_type_id);
            remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
            true
        }
        _ => false,
    }
}
