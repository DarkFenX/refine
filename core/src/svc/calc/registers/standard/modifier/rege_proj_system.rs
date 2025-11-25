use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, RawModifier,
        registers::standard::{
            data::StandardRegister,
            func::{add_cmod, remove_cmod},
        },
    },
    ud::UItem,
};

impl StandardRegister {
    pub(super) fn proj_system_mod(&mut self, rmod: RawModifier, projectee_item: &UItem) -> Option<CtxModifier> {
        {
            let projectee_ship = match projectee_item {
                UItem::Ship(projectee_ship) => projectee_ship,
                _ => return None,
            };
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind);
                    add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                AffecteeFilter::LocSrq(loc, srq_type_id) if let Ok(loc_kind) = loc.try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                AffecteeFilter::OwnSrq(srq_type_id) => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, srq_type_id);
                    add_cmod(&mut self.cmods.own_srq, key, cmod, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                _ => None,
            }
        }
    }
    pub(super) fn unproj_system_mod(&mut self, rmod: RawModifier, projectee_item: &UItem) -> Option<CtxModifier> {
        let projectee_ship = match projectee_item {
            UItem::Ship(projectee_ship) => projectee_ship,
            _ => return None,
        };
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, item_grp_id);
                remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) if let Ok(loc_kind) = loc.try_into() => {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, srq_type_id);
                remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let fit_key = projectee_ship.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, srq_type_id);
                remove_cmod(&mut self.cmods.own_srq, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            _ => None,
        }
    }
}
