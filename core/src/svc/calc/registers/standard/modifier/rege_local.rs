use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModifier,
        registers::standard::{
            StandardRegister,
            func::{add_cmod, remove_cmod},
        },
    },
    ud::UItem,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_local_mod(&mut self, item: &UItem, rmod: RawModifier) -> Option<CtxModifier> {
        let cmod = match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Item => {
                    let cmod = CtxModifier::from_raw(rmod);
                    add_cmod(
                        &mut self.cmods_direct,
                        cmod.raw.affector_espec.item_key,
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                Location::Other => {
                    let cmod = CtxModifier::from_raw(rmod);
                    add_cmod(
                        &mut self.cmods_other,
                        cmod.raw.affector_espec.item_key,
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                _ => {
                    let loc_kind = loc.try_into().ok()?;
                    let fit_key = item.get_fit_key()?;
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                    add_cmod(
                        &mut self.cmods_root,
                        (fit_key, loc_kind),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
            },
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                add_cmod(&mut self.cmods_loc, (fit_key, loc_kind), cmod, &mut self.cmods_by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                add_cmod(
                    &mut self.cmods_loc_grp,
                    (fit_key, loc_kind, item_grp_id),
                    cmod,
                    &mut self.cmods_by_aspec,
                );
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                add_cmod(
                    &mut self.cmods_loc_srq,
                    (fit_key, loc_kind, srq_type_id),
                    cmod,
                    &mut self.cmods_by_aspec,
                );
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                add_cmod(
                    &mut self.cmods_own_srq,
                    (fit_key, srq_type_id),
                    cmod,
                    &mut self.cmods_by_aspec,
                );
                Some(cmod)
            }
        };
        // If we received a modifier with context, it means that raw modifier was valid
        if cmod.is_some() {
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
        cmod
    }
    pub(in crate::svc::calc) fn unreg_local_mod(&mut self, item: &UItem, rmod: RawModifier) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Item => {
                    let cmod = CtxModifier::from_raw(rmod);
                    remove_cmod(
                        &mut self.cmods_direct,
                        cmod.raw.affector_espec.item_key,
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                Location::Other => {
                    let cmod = CtxModifier::from_raw(rmod);
                    remove_cmod(
                        &mut self.cmods_other,
                        cmod.raw.affector_espec.item_key,
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                _ => {
                    let loc_kind = loc.try_into().ok()?;
                    let fit_key = item.get_fit_key()?;
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                    remove_cmod(
                        &mut self.cmods_root,
                        (fit_key, loc_kind),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
            },
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                remove_cmod(
                    &mut self.cmods_loc,
                    (fit_key, loc_kind),
                    &cmod,
                    &mut self.cmods_by_aspec,
                );
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                remove_cmod(
                    &mut self.cmods_loc_grp,
                    (fit_key, loc_kind, item_grp_id),
                    &cmod,
                    &mut self.cmods_by_aspec,
                );
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                remove_cmod(
                    &mut self.cmods_loc_srq,
                    (fit_key, loc_kind, srq_type_id),
                    &cmod,
                    &mut self.cmods_by_aspec,
                );
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let fit_key = item.get_fit_key()?;
                let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                remove_cmod(
                    &mut self.cmods_own_srq,
                    (fit_key, srq_type_id),
                    &cmod,
                    &mut self.cmods_by_aspec,
                );
                Some(cmod)
            }
        }
    }
}
