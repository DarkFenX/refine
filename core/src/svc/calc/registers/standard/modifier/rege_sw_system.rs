use crate::{
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, RawModifier,
            registers::standard::{
                StandardRegister,
                modifier::func::{add_cmod, remove_cmod},
            },
        },
    },
    ud::UFitId,
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
            AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(loc, srq_type_aid) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::OwnSrq(srq_type_aid) => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, srq_type_aid);
                    add_cmod(&mut self.cmods.own_srq, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            _ => false,
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
            AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let key = (fit_uid, loc_kind);
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocSrq(loc, srq_type_aid) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::OwnSrq(srq_type_aid) => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                    let key = (fit_uid, srq_type_aid);
                    remove_cmod(&mut self.cmods.own_srq, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            _ => (),
        }
        // Only modifiers which passed validation during registration should reach this function, so
        // we don't do extra validation and just remove them
        self.rmods_sw_system.remove(&rmod);
    }
    // No need to return any ctx modifiers here, since fits being added have no items
    pub(in crate::svc::calc) fn reg_fit_for_sw(&mut self, fit_uid: UFitId) {
        for rmod in self.rmods_sw_system.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(loc, srq_type_aid) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::OwnSrq(srq_type_aid) => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, srq_type_aid);
                    add_cmod(&mut self.cmods.own_srq, key, cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            }
        }
    }
    // No need to return any ctx modifiers here, since fits being removed have no items
    pub(in crate::svc::calc) fn unreg_fit_for_sw(&mut self, fit_uid: UFitId) {
        for rmod in self.rmods_sw_system.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind);
                    remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(loc, srq_type_aid) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::OwnSrq(srq_type_aid) => {
                    let cmod = CtxModifier::new_with_fit(*rmod, fit_uid);
                    let key = (fit_uid, srq_type_aid);
                    remove_cmod(&mut self.cmods.own_srq, key, &cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            }
        }
    }
}
