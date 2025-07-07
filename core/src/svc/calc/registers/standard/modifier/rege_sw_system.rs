use super::{add_cmod, remove_cmod};
use crate::{
    def::FitKey,
    svc::{
        SvcCtx,
        calc::{AffecteeFilter, CtxModifier, RawModifier, registers::StandardRegister},
    },
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_sw_system_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        rmod: RawModifier,
    ) {
        reuse_cmods.clear();
        let valid = match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_root,
                            (fit_key, loc_kind),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                        reuse_cmods.push(cmod);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        add_cmod(&mut self.cmods_loc, (fit_key, loc_kind), cmod, &mut self.cmods_by_aspec);
                        reuse_cmods.push(cmod);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_loc_grp,
                            (fit_key, loc_kind, a_item_grp_id),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                        reuse_cmods.push(cmod);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_loc_srq,
                            (fit_key, loc_kind, srq_a_item_id),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                        reuse_cmods.push(cmod);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                reuse_cmods.reserve(ctx.uad.fits.len());
                for fit_key in ctx.uad.fits.keys() {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                    add_cmod(
                        &mut self.cmods_own_srq,
                        (fit_key, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                true
            }
        };
        if valid {
            self.rmods_sw_system.insert(rmod);
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
    }
    pub(in crate::svc::calc) fn unreg_sw_system_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        rmod: RawModifier,
    ) {
        reuse_cmods.clear();
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                if let Ok(loc_kind) = loc.try_into() {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_root,
                            &(fit_key, loc_kind),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
            }
            AffecteeFilter::Loc(loc) => {
                if let Ok(loc_kind) = loc.try_into() {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc,
                            &(fit_key, loc_kind),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
            }
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                if let Ok(loc_kind) = loc.try_into() {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc_grp,
                            &(fit_key, loc_kind, a_item_grp_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
            }
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                if let Ok(loc_kind) = loc.try_into() {
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for fit_key in ctx.uad.fits.keys() {
                        let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc_srq,
                            &(fit_key, loc_kind, srq_a_item_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                reuse_cmods.reserve(ctx.uad.fits.len());
                for fit_key in ctx.uad.fits.keys() {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fit_key);
                    remove_cmod(
                        &mut self.cmods_own_srq,
                        &(fit_key, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
            }
        }
        self.rmods_sw_system.remove(&rmod);
    }
    // No need to return any ctx modifiers here, since fits being added have no items
    pub(in crate::svc::calc) fn reg_fit_for_sw(&mut self, fit_key: FitKey) {
        for rmod in self.rmods_sw_system.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_root,
                            (fit_key, loc_kind),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                    }
                }
                AffecteeFilter::Loc(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        add_cmod(&mut self.cmods_loc, (fit_key, loc_kind), cmod, &mut self.cmods_by_aspec);
                    }
                }
                AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_loc_grp,
                            (fit_key, loc_kind, a_item_grp_id),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        add_cmod(
                            &mut self.cmods_loc_srq,
                            (fit_key, loc_kind, srq_a_item_id),
                            cmod,
                            &mut self.cmods_by_aspec,
                        );
                    }
                }
                AffecteeFilter::OwnSrq(srq_a_item_id) => {
                    let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                    add_cmod(
                        &mut self.cmods_own_srq,
                        (fit_key, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
            }
        }
    }
    // No need to return any ctx modifiers here, since fits being removed have no items
    pub(in crate::svc::calc) fn unreg_fit_for_sw(&mut self, fit_key: FitKey) {
        for rmod in self.rmods_sw_system.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_root,
                            &(fit_key, loc_kind),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                    }
                }
                AffecteeFilter::Loc(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc,
                            &(fit_key, loc_kind),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc_grp,
                            &(fit_key, loc_kind, a_item_grp_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                        remove_cmod(
                            &mut self.cmods_loc_srq,
                            &(fit_key, loc_kind, srq_a_item_id),
                            &cmod,
                            &mut self.cmods_by_aspec,
                        );
                    }
                }
                AffecteeFilter::OwnSrq(srq_a_item_id) => {
                    let cmod = CtxModifier::from_raw_with_fit(*rmod, fit_key);
                    remove_cmod(
                        &mut self.cmods_own_srq,
                        &(fit_key, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
            }
        }
    }
}
