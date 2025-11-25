use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModifier,
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
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::Target) => {
                let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
                add_cmod(&mut self.cmods.direct, projectee_key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(Location::Target) if let UItem::Ship(projectee_ship) = projectee_item => {
                match projectee_ship.get_kind().try_into() {
                    Ok(loc_kind) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, loc_kind);
                        add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocGrp(Location::Target, item_grp_id)
                if let UItem::Ship(projectee_ship) = projectee_item =>
            {
                match projectee_ship.get_kind().try_into() {
                    Ok(loc_kind) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, loc_kind, item_grp_id);
                        add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocSrq(Location::Target, srq_type_id)
                if let UItem::Ship(projectee_ship) = projectee_item =>
            {
                match projectee_ship.get_kind().try_into() {
                    Ok(loc_kind) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, loc_kind, srq_type_id);
                        add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.add_entry(projectee_key, rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.add_entry(projectee_key, rmod);
                        None
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_type_id) if let UItem::Ship(projectee_ship) = projectee_item => {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                let key = (fit_key, srq_type_id);
                add_cmod(&mut self.cmods.own_srq, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            _ => None,
        }
    }
    pub(super) fn unproj_target_mod(
        &mut self,
        rmod: RawModifier,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::Target) => {
                let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
                remove_cmod(&mut self.cmods.direct, projectee_key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(Location::Target) if let UItem::Ship(projectee_ship) = projectee_item => {
                match projectee_ship.get_kind().try_into() {
                    Ok(loc_kind) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, loc_kind);
                        remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocGrp(Location::Target, item_grp_id)
                if let UItem::Ship(projectee_ship) = projectee_item =>
            {
                match projectee_ship.get_kind().try_into() {
                    Ok(loc_kind) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, loc_kind, item_grp_id);
                        remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                }
            }
            AffecteeFilter::LocSrq(Location::Target, srq_type_id)
                if let UItem::Ship(projectee_ship) = projectee_item =>
            {
                match projectee_ship.get_kind().try_into() {
                    Ok(loc_kind) => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        let key = (fit_key, loc_kind, srq_type_id);
                        remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                        self.rmods_proj_active.remove_entry(projectee_key, &rmod);
                        Some(cmod)
                    }
                    _ => {
                        self.rmods_proj_inactive.remove_entry(projectee_key, &rmod);
                        None
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_type_id) if let UItem::Ship(projectee_ship) = projectee_item => {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                let key = (fit_key, srq_type_id);
                remove_cmod(&mut self.cmods.own_srq, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            _ => None,
        }
    }
}

pub(super) fn query_target_mod(
    rmod: RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::Target) => {
                let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
                Some(cmod)
            }
            AffecteeFilter::Loc(Location::Target)
            | AffecteeFilter::LocGrp(Location::Target, _)
            | AffecteeFilter::LocSrq(Location::Target, _)
                if let UItem::Ship(projectee_ship) = projectee_item =>
            {
                match projectee_ship.get_kind() {
                    UShipKind::Ship | UShipKind::Structure => {
                        let fit_key = projectee_ship.get_fit_key();
                        let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
                        Some(cmod)
                    }
                    _ => None,
                }
            }
            AffecteeFilter::OwnSrq(_) if let UItem::Ship(projectee_ship) = projectee_item => {
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
pub(super) fn load_affectee_for_proj_target(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Loc(loc)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind);
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
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, item_grp_id);
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
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    add_cmod(&mut cdata.loc_srq, key, cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}
pub(super) fn unload_affectee_for_proj_target(
    cdata: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Loc(loc)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind);
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
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, item_grp_id);
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
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    remove_cmod(&mut cdata.loc_srq, key, &cmod, &mut cdata.by_aspec);
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}
