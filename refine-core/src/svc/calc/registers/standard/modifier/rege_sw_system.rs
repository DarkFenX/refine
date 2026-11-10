use crate::{
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, RawModId, RawModTemplates, RawModifier,
            registers::{
                RawModStorage, RegOp,
                standard::{StandardRegister, modifier::func::process_cmod},
            },
        },
    },
    ud::UFitId,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn process_sw_system_mod<const OP: RegOp>(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> bool {
        let valid = match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.root, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(loc, srq_type_aid) if let Ok(loc_kind) = loc.try_into() => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::OwnSrq(srq_type_aid) => {
                reuse_cmods.reserve(ctx.u_data.fits.len());
                for fit_uid in ctx.u_data.fits.keys() {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.own_srq, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            _ => false,
        };
        if valid {
            match OP {
                RegOp::Reg => self.rmods_sw_system.insert(rmod_id),
                RegOp::Unreg => self.rmods_sw_system.remove(&rmod_id),
            };
        }
        valid
    }
    // No need to return any ctx modifiers here, since fits being added or removed have no items
    pub(in crate::svc::calc) fn process_fit_for_sw<const OP: RegOp>(
        &mut self,
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        fit_uid: UFitId,
    ) {
        for &rmod_id in self.rmods_sw_system.iter() {
            let rmod = rmods.get_rmod(templates, rmod_id);
            match rmod.affectee_filter {
                AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.root, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(loc, srq_type_aid) if let Ok(loc_kind) = loc.try_into() => {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::OwnSrq(srq_type_aid) => {
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.own_srq, &mut self.cmods.by_aspec);
                }
                _ => (),
            }
        }
    }
}
