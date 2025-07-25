use super::{add_cmod, remove_cmod};
use crate::{
    svc::calc::{AffecteeFilter, CtxModifier, Location, RawModifier, registers::StandardRegister},
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
                _ => match loc.try_into() {
                    Ok(loc_kind) => match item.get_fit_key() {
                        Some(fit_key) => {
                            let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                            add_cmod(
                                &mut self.cmods_root,
                                (fit_key, loc_kind),
                                cmod,
                                &mut self.cmods_by_aspec,
                            );
                            Some(cmod)
                        }
                        None => None,
                    },
                    _ => None,
                },
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        add_cmod(&mut self.cmods_loc, (fit_key, loc_kind), cmod, &mut self.cmods_by_aspec);
                        Some(cmod)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_loc_grp,
                            (fit_key, loc_kind, a_item_grp_id),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                        Some(cmod)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_loc_srq,
                            (fit_key, loc_kind, srq_a_item_id),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                        Some(cmod)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match item.get_fit_key() {
                Some(fit_key) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                    add_cmod(
                        &mut self.cmods_own_srq,
                        (fit_key, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                None => None,
            },
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
                        &cmod.raw.affector_espec.item_key,
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                Location::Other => {
                    let cmod = CtxModifier::from_raw(rmod);
                    remove_cmod(
                        &mut self.cmods_other,
                        &cmod.raw.affector_espec.item_key,
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                _ => match loc.try_into() {
                    Ok(loc_kind) => match item.get_fit_key() {
                        Some(fit_key) => {
                            let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                            remove_cmod(
                                &mut self.cmods_root,
                                &(fit_key, loc_kind),
                                &cmod,
                                &mut self.cmods_by_aspec,
                            );
                            Some(cmod)
                        }
                        None => None,
                    },
                    _ => None,
                },
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc,
                            &(fit_key, loc_kind),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        Some(cmod)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc_grp,
                            &(fit_key, loc_kind, a_item_grp_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        Some(cmod)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc_srq,
                            &(fit_key, loc_kind, srq_a_item_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        Some(cmod)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match item.get_fit_key() {
                Some(fit_key) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                    remove_cmod(
                        &mut self.cmods_own_srq,
                        &(fit_key, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    Some(cmod)
                }
                None => None,
            },
        }
    }
}
